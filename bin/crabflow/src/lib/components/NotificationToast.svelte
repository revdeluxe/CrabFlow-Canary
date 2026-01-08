<script>
    import { notifications } from '$lib/stores/notifications';
    import { fade, fly } from 'svelte/transition';
    import { onDestroy } from 'svelte';

    let toasts = [];

    const unsubscribe = notifications.subscribe(value => {
        toasts = value;
    });

    onDestroy(unsubscribe);
</script>

<div class="toast-container">
    {#each toasts as toast (toast.id)}
        <div 
            class="toast-item {toast.level}" 
            in:fly="{{ y: 20, duration: 300 }}" 
            out:fade
        >
            <div class="toast-header">
                <strong class="mr-auto">{toast.title}</strong>
                <small>Just now</small>
            </div>
            <div class="toast-body">
                {toast.message}
            </div>
        </div>
    {/each}
</div>

<style>
    .toast-container {
        position: fixed;
        bottom: 20px;
        right: 20px;
        z-index: 9999;
        display: flex;
        flex-direction: column;
        gap: 10px;
    }

    .toast-item {
        min-width: 300px;
        max-width: 400px;
        background: white;
        border-radius: 4px;
        box-shadow: 0 4px 12px rgba(0,0,0,0.15);
        overflow: hidden;
        border-left: 4px solid #ccc;
    }

    .toast-item.info { border-left-color: #17a2b8; }
    .toast-item.success { border-left-color: #28a745; }
    .toast-item.warning { border-left-color: #ffc107; }
    .toast-item.error { border-left-color: #dc3545; }

    .toast-header {
        padding: 0.5rem 0.75rem;
        background-color: rgba(0,0,0,0.03);
        border-bottom: 1px solid rgba(0,0,0,0.05);
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .toast-body {
        padding: 0.75rem;
    }
</style>
