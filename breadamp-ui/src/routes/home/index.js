import { useState } from "preact/hooks";
import "98.css";

const Home = () => {
  const [msg, setMsg] = useState("stopped");
  const [isPlaying, setIsPlaying] = useState(false);
  const prevTrack = () => setMsg("prev");
  const nextTrack = () => setMsg("next");
  return (
    <div className="window-body">
      <p style={{ textAlign: "center" }}>{msg}</p>
      <div className="field-row" style={{ justifyContent: "center" }}>
        <button onClick={() => prevTrack()}>⏪</button>
        <button
          onClick={() => {
            setIsPlaying(!isPlaying);
            setMsg(!isPlaying ? "playing" : "stopped");
          }}
        >
          {isPlaying ? "⏹️" : "▶️"}
        </button>
        <button onClick={() => nextTrack()}>⏩</button>
      </div>
    </div>
  );
};

export default Home;
