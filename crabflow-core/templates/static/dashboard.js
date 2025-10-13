document.getElementById('refresh')?.addEventListener('click', fetchStatus);
document.getElementById('diag')?.addEventListener('click', runDiag);

async function fetchStatus(){
  try{
    const res = await fetch('/status.json');
    if(!res.ok) throw new Error('status fetch failed');
    const data = await res.json();
    // update DOM: simple example
    document.querySelector('.kv dd:nth-child(2)')?.textContent = data.cpu || '';
    const list = document.querySelector('.iflist');
    if(list){
      list.innerHTML = data.interfaces.map(i => `<li><code>${i}</code></li>`).join('');
    }
  }catch(e){
    console.error(e);
    alert('Failed to fetch status');
  }
}

async function runDiag(){
  const res = await fetch('/diag', { method: 'POST' });
  const txt = await res.text();
  alert('Diag: ' + txt);
}
