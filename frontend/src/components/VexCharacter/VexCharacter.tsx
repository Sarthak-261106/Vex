type Props = { blind: boolean; muted: boolean };
export function VexCharacter({ blind, muted }: Props) {
  const eyeClass = blind ? "eyes blind" : muted ? "eyes muted" : "eyes";
  return <div className="vex-character" aria-label="VEX desktop companion">
    <div className="antenna left" /><div className="antenna right" />
    <div className="head"><div className="visor"><div className={eyeClass}><i /><i /></div></div></div>
    <div className="neck" /><div className="body"><span>V</span><div className="core" /></div>
    <div className="base"><b /></div>
  </div>;
}
