type Action = "mute" | "blind" | "order" | "settings" | "exit";
type Props = { muted: boolean; blind: boolean; onAction: (action: Action) => void };
export function VexMenu({ muted, blind, onAction }: Props) {
  return <nav className="vex-menu" onClick={(e) => e.stopPropagation()} aria-label="VEX controls">
    <button onClick={() => onAction("mute")}>{muted ? "Unmute" : "Mute"}</button>
    <button onClick={() => onAction("blind")}>{blind ? "Blind — On" : "Enable Blind"}</button>
    <button onClick={() => onAction("order")}>Give Order</button>
    <button onClick={() => onAction("settings")}>Settings</button>
    <hr /><button className="danger" onClick={() => onAction("exit")}>Exit VEX</button>
  </nav>;
}
