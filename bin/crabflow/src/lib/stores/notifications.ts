import { writable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';

export interface Notification {
  id: number;
  title: string;
  message: string;
  level: 'info' | 'success' | 'warning' | 'error';
  timestamp: number;
}

const { subscribe, update } = writable<Notification[]>([]);

let nextId = 0;

export const notifications = {
  subscribe,
  add: (n: { title: string; message: string; level: string; timestamp?: number }) => {
    const id = nextId++;
    const fullNotification: Notification = {
      id,
      title: n.title,
      message: n.message,
      level: n.level as any,
      timestamp: n.timestamp || Date.now() / 1000,
    };
    
    update((list) => [...list, fullNotification]);
    
    // Auto remove after 5 seconds
    setTimeout(() => {
        update((list) => list.filter((item) => item.id !== id));
    }, 5000);
  },
  
  listenForEvents: async () => {
    try {
        await listen('notification-event', (event: any) => {
            const payload = event.payload;
            notifications.add(payload);
        });
        console.log("Listening for notification events...");
    } catch (e) {
        console.error("Failed to listen for notifications", e);
    }
  }
};
