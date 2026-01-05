<script>
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'

  let blacklist = []
  let newDomain = ""
  let loading = false
  
  // Modal states
  let exportFormat = 'json'
  let importUrl = ''
  let selectedFile = null
  let selectedFileName = ''
  let importing = false
  let activeTab = 'file'

  // Pagination & Search
  let searchQuery = ""
  let currentPage = 1
  let itemsPerPage = 50

  $: filteredBlacklist = blacklist.filter(d => d.toLowerCase().includes(searchQuery.toLowerCase()))
  $: totalPages = Math.ceil(filteredBlacklist.length / itemsPerPage)
  $: paginatedBlacklist = filteredBlacklist.slice((currentPage - 1) * itemsPerPage, currentPage * itemsPerPage)

  // Reset page when search changes
  $: if (searchQuery) currentPage = 1

  async function loadBlacklist() {
    loading = true
    try {
      blacklist = await invoke("get_blacklist")
    } catch (e) {
      alert("Failed to load blacklist: " + e)
    } finally {
      loading = false
    }
  }

  async function addDomain() {
    if (!newDomain) return
    try {
      await invoke("block_domain", { domain: newDomain })
      newDomain = ""
      loadBlacklist()
    } catch (e) {
      alert("Failed to block domain: " + e)
    }
  }

  async function removeDomain(domain) {
    if (!confirm(`Unblock ${domain}?`)) return
    try {
      await invoke("unblock_domain", { domain })
      loadBlacklist()
    } catch (e) {
      alert("Failed to unblock domain: " + e)
    }
  }

  function performExport() {
    let content = ""
    let type = "text/plain"
    let ext = "txt"

    switch (exportFormat) {
        case 'json':
            content = JSON.stringify(blacklist, null, 2)
            type = "application/json"
            ext = "json"
            break
        case 'text':
            content = blacklist.join('\n')
            break
        case 'hosts':
            content = blacklist.map(d => `0.0.0.0 ${d}`).join('\n')
            break
        case 'abp':
            content = blacklist.map(d => `||${d}^`).join('\n')
            break
    }

    const blob = new Blob([content], { type })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `blacklist-${Date.now()}.${ext}`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  }

  function handleFileSelect(event) {
      selectedFile = event.target.files[0]
      selectedFileName = selectedFile ? selectedFile.name : ''
  }

  async function performImport() {
      importing = true
      let content = ""

      try {
          if (activeTab === 'url') {
              if (!importUrl) throw new Error("URL is required")
              const res = await fetch(importUrl)
              if (!res.ok) throw new Error(`Failed to fetch: ${res.statusText}`)
              content = await res.text()
          } else {
              if (!selectedFile) throw new Error("File is required")
              content = await selectedFile.text()
          }

          // Parse content
          let domains = []
          try {
              const json = JSON.parse(content)
              if (Array.isArray(json)) domains = json
          } catch {
              // Fallback: Line by line
              domains = content.split('\n')
                .map(l => l.trim())
                .filter(l => l && !l.startsWith('#') && !l.startsWith('!'))
                
              // Clean up hosts file format (0.0.0.0 domain)
              domains = domains.map(d => {
                  const parts = d.split(/\s+/)
                  if (parts.length >= 2 && (parts[0] === '0.0.0.0' || parts[0] === '127.0.0.1')) {
                      return parts[1]
                  }
                  return d
              })
              
              // Clean up ABP format (||domain^)
              domains = domains.map(d => {
                  if (d.startsWith('||') && d.endsWith('^')) {
                      return d.slice(2, -1)
                  }
                  return d
              })
          }

          if (domains.length > 0) {
              const count = await invoke("import_blacklist", { domains })
              alert(`Successfully imported ${count} new domains.`)
              loadBlacklist()
              // Reset inputs
              importUrl = ''
              selectedFile = null
              selectedFileName = ''
              // Ideally close modal here, but we rely on user or bootstrap
          } else {
              alert("No valid domains found.")
          }

      } catch (e) {
          alert("Import failed: " + e.message)
      } finally {
          importing = false
      }
  }

  onMount(() => {
    loadBlacklist()
  })
</script>

<section class="content-header">
  <div class="container-fluid">
    <div class="row mb-2">
      <div class="col-sm-6">
        <h1>Ad-Block / Blacklist</h1>
      </div>
    </div>
  </div>
</section>

<section class="content">
  <div class="container-fluid">
    <div class="card">
      <div class="card-header">
        <h3 class="card-title">Blocked Domains</h3>
        <div class="card-tools">
          <button class="btn btn-tool" data-toggle="modal" data-target="#exportModal" title="Export">
            <i class="fas fa-download"></i> Export
          </button>
          <button class="btn btn-tool" data-toggle="modal" data-target="#importModal" title="Import">
            <i class="fas fa-upload"></i> Import
          </button>
        </div>
      </div>
      <div class="card-body">
        <div class="row mb-3">
            <div class="col-md-8">
                <div class="input-group">
                    <input type="text" class="form-control" placeholder="Enter domain to block (e.g. ads.example.com)" bind:value={newDomain} on:keydown={(e) => e.key === 'Enter' && addDomain()}>
                    <div class="input-group-append">
                        <button class="btn btn-danger" type="button" on:click={addDomain}>Block Domain</button>
                    </div>
                </div>
            </div>
            <div class="col-md-4">
                <div class="input-group">
                    <input type="text" class="form-control" placeholder="Search domains..." bind:value={searchQuery}>
                    <div class="input-group-append">
                        <span class="input-group-text"><i class="fas fa-search"></i></span>
                    </div>
                </div>
            </div>
        </div>

        <div class="table-responsive p-0" style="height: 600px;">
          <table class="table table-bordered table-hover table-sm table-head-fixed text-nowrap">
            <thead>
              <tr>
                <th>Domain ({filteredBlacklist.length})</th>
                <th style="width: 100px" class="text-center">Action</th>
              </tr>
            </thead>
            <tbody>
              {#if loading}
                <tr><td colspan="2" class="text-center">Loading...</td></tr>
              {:else if filteredBlacklist.length === 0}
                <tr><td colspan="2" class="text-center">No domains found.</td></tr>
              {:else}
                {#each paginatedBlacklist as domain}
                  <tr>
                    <td class="align-middle">{domain}</td>
                    <td class="text-center">
                      <button class="btn btn-xs btn-success" on:click={() => removeDomain(domain)}>
                        Unblock
                      </button>
                    </td>
                  </tr>
                {/each}
              {/if}
            </tbody>
          </table>
        </div>

        <!-- Pagination -->
        {#if totalPages > 1}
        <div class="d-flex justify-content-between align-items-center mt-2">
            <div>
                Showing {(currentPage - 1) * itemsPerPage + 1} to {Math.min(currentPage * itemsPerPage, filteredBlacklist.length)} of {filteredBlacklist.length} entries
            </div>
            <ul class="pagination pagination-sm m-0">
                <li class="page-item" class:disabled={currentPage === 1}>
                    <a class="page-link" href="#" on:click|preventDefault={() => currentPage--}>&laquo;</a>
                </li>
                
                <!-- Simple pagination logic: show current, prev, next -->
                {#if currentPage > 2}
                    <li class="page-item"><a class="page-link" href="#" on:click|preventDefault={() => currentPage = 1}>1</a></li>
                    {#if currentPage > 3}<li class="page-item disabled"><span class="page-link">...</span></li>{/if}
                {/if}

                {#if currentPage > 1}
                    <li class="page-item"><a class="page-link" href="#" on:click|preventDefault={() => currentPage--}>{currentPage - 1}</a></li>
                {/if}

                <li class="page-item active"><span class="page-link">{currentPage}</span></li>

                {#if currentPage < totalPages}
                    <li class="page-item"><a class="page-link" href="#" on:click|preventDefault={() => currentPage++}>{currentPage + 1}</a></li>
                {/if}

                {#if currentPage < totalPages - 1}
                    {#if currentPage < totalPages - 2}<li class="page-item disabled"><span class="page-link">...</span></li>{/if}
                    <li class="page-item"><a class="page-link" href="#" on:click|preventDefault={() => currentPage = totalPages}>{totalPages}</a></li>
                {/if}

                <li class="page-item" class:disabled={currentPage === totalPages}>
                    <a class="page-link" href="#" on:click|preventDefault={() => currentPage++}>&raquo;</a>
                </li>
            </ul>
        </div>
        {/if}
      </div>
    </div>
  </div>
</section>

<!-- Export Modal -->
<div class="modal fade" id="exportModal" tabindex="-1" role="dialog" aria-labelledby="exportModalLabel" aria-hidden="true">
  <div class="modal-dialog" role="document">
    <div class="modal-content">
      <div class="modal-header">
        <h5 class="modal-title" id="exportModalLabel">Export Blacklist</h5>
        <button type="button" class="close" data-dismiss="modal" aria-label="Close">
          <span aria-hidden="true">&times;</span>
        </button>
      </div>
      <div class="modal-body">
        <div class="form-group">
          <label>Format</label>
          <select class="form-control" bind:value={exportFormat}>
            <option value="json">JSON</option>
            <option value="text">Plain Text (One per line)</option>
            <option value="hosts">Hosts File (0.0.0.0)</option>
            <option value="abp">AdBlock Plus</option>
          </select>
        </div>
      </div>
      <div class="modal-footer">
        <button type="button" class="btn btn-secondary" data-dismiss="modal">Close</button>
        <button type="button" class="btn btn-primary" on:click={performExport}>Export</button>
      </div>
    </div>
  </div>
</div>

<!-- Import Modal -->
<div class="modal fade" id="importModal" tabindex="-1" role="dialog" aria-labelledby="importModalLabel" aria-hidden="true">
  <div class="modal-dialog" role="document">
    <div class="modal-content">
      <div class="modal-header">
        <h5 class="modal-title" id="importModalLabel">Import Blacklist</h5>
        <button type="button" class="close" data-dismiss="modal" aria-label="Close">
          <span aria-hidden="true">&times;</span>
        </button>
      </div>
      <div class="modal-body">
        <ul class="nav nav-tabs" id="importTab" role="tablist">
          <li class="nav-item">
            <a class="nav-link" class:active={activeTab === 'file'} href="#" on:click|preventDefault={() => activeTab = 'file'}>File Upload</a>
          </li>
          <li class="nav-item">
            <a class="nav-link" class:active={activeTab === 'url'} href="#" on:click|preventDefault={() => activeTab = 'url'}>URL</a>
          </li>
        </ul>
        <div class="tab-content mt-3">
          {#if activeTab === 'file'}
             <div class="custom-file">
                <input type="file" class="custom-file-input" id="importFile" on:change={handleFileSelect}>
                <label class="custom-file-label" for="importFile">{selectedFileName || "Choose file"}</label>
             </div>
          {:else}
             <div class="form-group">
                <label>URL (Raw Text/JSON)</label>
                <input type="text" class="form-control" placeholder="https://example.com/list.txt" bind:value={importUrl}>
                <small class="form-text text-muted">Must be a raw text file or JSON.</small>
             </div>
          {/if}
        </div>
      </div>
      <div class="modal-footer">
        <button type="button" class="btn btn-secondary" data-dismiss="modal">Close</button>
        <button type="button" class="btn btn-primary" on:click={performImport} disabled={importing}>
            {importing ? 'Importing...' : 'Import'}
        </button>
      </div>
    </div>
  </div>
</div>
