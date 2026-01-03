// src/ui/EventCanvas.tsx
import { useEffect, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api";

type NodeId =
  | "welcome"
  | "setup_started"
  | "setup_completed"
  | "dashboard"
  | "consent_prompt"
  | "consent_granted"
  | "consent_denied"
  | "flow_toggle_on"
  | "flow_toggle_off";

type Edge = { from: NodeId; to: NodeId; label?: string };
type Node = { id: NodeId; x: number; y: number; color: string };

const nodes: Node[] = [
  { id: "welcome", x: 100, y: 120, color: "#444" },
  { id: "setup_started", x: 320, y: 80, color: "#1e90ff" },
  { id: "setup_completed", x: 560, y: 80, color: "#2ecc71" },
  { id: "dashboard", x: 560, y: 220, color: "#8e44ad" },
  { id: "consent_prompt", x: 320, y: 220, color: "#f39c12" },
  { id: "consent_granted", x: 320, y: 360, color: "#27ae60" },
  { id: "consent_denied", x: 320, y: 360, color: "#c0392b" },
  { id: "flow_toggle_on", x: 780, y: 220, color: "#2ecc71" },
  { id: "flow_toggle_off", x: 780, y: 320, color: "#e74c3c" },
];

const edges: Edge[] = [
  { from: "welcome", to: "setup_started", label: "Begin Setup" },
  { from: "setup_started", to: "setup_completed", label: "Init OK" },
  { from: "setup_completed", to: "dashboard", label: "Open Dashboard" },
  { from: "dashboard", to: "consent_prompt", label: "ML feature" },
  { from: "consent_prompt", to: "consent_granted", label: "Grant" },
  { from: "consent_prompt", to: "consent_denied", label: "Decline" },
  { from: "dashboard", to: "flow_toggle_on", label: "Start Flow" },
  { from: "flow_toggle_on", to: "flow_toggle_off", label: "Stop Flow" },
  { from: "flow_toggle_off", to: "flow_toggle_on", label: "Start Flow" },
];

function draw(ctx: CanvasRenderingContext2D, nodes: Node[], edges: Edge[], active?: NodeId) {
  ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
  ctx.lineWidth = 2;
  ctx.font = "12px system-ui";

  // Draw edges
  edges.forEach(({ from, to, label }) => {
    const a = nodes.find(n => n.id === from)!;
    const b = nodes.find(n => n.id === to)!;
    ctx.strokeStyle = "#8599a4";
    ctx.beginPath();
    ctx.moveTo(a.x, a.y);
    ctx.lineTo(b.x, b.y);
    ctx.stroke();

    // Arrowhead
    const angle = Math.atan2(b.y - a.y, b.x - a.x);
    const ah = 8;
    ctx.beginPath();
    ctx.moveTo(b.x, b.y);
    ctx.lineTo(b.x - ah * Math.cos(angle - Math.PI / 6), b.y - ah * Math.sin(angle - Math.PI / 6));
    ctx.lineTo(b.x - ah * Math.cos(angle + Math.PI / 6), b.y - ah * Math.sin(angle + Math.PI / 6));
    ctx.closePath();
    ctx.fillStyle = "#8599a4";
    ctx.fill();

    // Edge label
    if (label) {
      ctx.fillStyle = "#5d6d74";
      ctx.fillText(label, (a.x + b.x) / 2 + 6, (a.y + b.y) / 2 - 6);
    }
  });

  // Draw nodes
  nodes.forEach(n => {
    ctx.beginPath();
    ctx.arc(n.x, n.y, 18, 0, 2 * Math.PI);
    ctx.fillStyle = active === n.id ? "#111" : n.color;
    ctx.fill();
    ctx.strokeStyle = "#222";
    ctx.stroke();

    ctx.fillStyle = "#fff";
    ctx.textAlign = "center";
    ctx.fillText(n.id, n.x, n.y + 36); // label under node
  });
}

export default function EventCanvas() {
  const ref = useRef<HTMLCanvasElement>(null);
  const [active, setActive] = useState<NodeId>("welcome");

  useEffect(() => {
    const canvas = ref.current!;
    const ctx = canvas.getContext("2d")!;
    draw(ctx, nodes, edges, active);

    const unlistenPromise = listen("crabflow://render", (event) => {
      const payload = event.payload as { kind: NodeId; data: any };
      setActive(payload.kind);
      draw(ctx, nodes, edges, payload.kind);
    });

    // Click on node to invoke related action
    const onClick = (e: MouseEvent) => {
      const rect = canvas.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;
      const hit = nodes.find(n => Math.hypot(n.x - x, n.y - y) < 18);
      if (!hit) return;

      // Map node clicks to backend invokes
      const map: Record<NodeId, string | null> = {
        welcome: "begin_setup",
        setup_started: null,
        setup_completed: "open_dashboard",
        dashboard: "prompt_consent",
        consent_prompt: null,
        consent_granted: "enable_ml",
        consent_denied: "disable_ml",
        flow_toggle_on: "stop_flow",
        flow_toggle_off: "start_flow",
      };

      const cmd = map[hit.id];
      if (cmd) invoke(cmd).catch(console.error);
    };

    canvas.addEventListener("click", onClick);
    return () => {
      canvas.removeEventListener("click", onClick);
      unlistenPromise.then(u => u());
    };
  }, [active]);

  return <canvas ref={ref} width={900} height={500} style={{ width: 900, height: 500, border: "1px solid #2b2f33" }} />;
}
