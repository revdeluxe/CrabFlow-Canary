<script>
  import { onMount } from 'svelte';
  import '../app.css';
  import NotificationToast from '$lib/components/NotificationToast.svelte';
  import { notifications } from '$lib/stores/notifications';

  onMount(() => {
    notifications.listenForEvents();
  });
</script>

<NotificationToast />
<slot />
