import { useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { Caption } from "./components/Captions/Caption";
import { VexCharacter } from "./components/VexCharacter/VexCharacter";
import { VexMenu } from "./components/VexMenu/VexMenu";

type MenuAction = "mute" | "blind" | "order" | "settings" | "exit";
const appWindow = getCurrentWindow();

export function App() {
  const [menuOpen, setMenuOpen] = useState(false);
  const [muted, setMuted] = useState(false);
  const [blind, setBlind] = useState(true);
  const [caption, setCaption] = useState("Online.");

  useEffect(() => {
    const savePosition = () => void appWindow.outerPosition().then((p) => invoke("save_window_position", { x: p.x, y: p.y }));
    const dismissMenu = () => setMenuOpen(false);
    window.addEventListener("blur", dismissMenu);
    window.addEventListener("mouseup", savePosition);
    return () => { window.removeEventListener("mouseup", savePosition); window.removeEventListener("blur", dismissMenu); };
  }, []);

  async function handleAction(action: MenuAction) {
    setMenuOpen(false);
    if (action === "mute") { setMuted((value) => !value); setCaption(muted ? "Voice output restored." : "Voice output muted. Captions remain on."); }
    if (action === "blind") { setBlind((value) => !value); setCaption(blind ? "Vision remains off until explicitly enabled." : "Blind mode active. Screen access blocked."); }
    if (action === "order") { setCaption("Give me an order. Text brain arrives in Phase 3."); }
    if (action === "settings") { setCaption("Settings panel is scheduled for the next desktop iteration."); }
    if (action === "exit") await appWindow.close();
  }

  return <main className="vex-shell" onClick={() => menuOpen && setMenuOpen(false)}>
    <Caption text={caption} muted={muted} blind={blind} />
    <section className="vex-stage" onMouseDown={(event) => { if (event.button === 0) void appWindow.startDragging(); }}>
      <VexCharacter blind={blind} muted={muted} />
    </section>
    <button aria-label="Open VEX menu" className="menu-trigger" onMouseDown={(e) => e.stopPropagation()} onClick={(e) => { e.stopPropagation(); setMenuOpen((value) => !value); }}>•••</button>
    {menuOpen && <VexMenu muted={muted} blind={blind} onAction={handleAction} />}
  </main>;
}
