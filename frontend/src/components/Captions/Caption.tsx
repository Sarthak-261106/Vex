type Props = { text: string; muted: boolean; blind: boolean };
export function Caption({ text, muted, blind }: Props) {
  return <div className="caption" role="status"><span className="caption-dot" />{text}<small>{blind ? "BLIND" : muted ? "MUTED" : "LOCAL"}</small></div>;
}
