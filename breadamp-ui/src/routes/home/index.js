import { useState } from "preact/hooks";
import "98.css";

/// TODO TODO TODO
/// TODO TODO TODO
/// TODO TODO TODO
/// TODO TODO TODO
/// TODO TODO TODO
/// TODO TODO TODO
/// TODO TODO TODO
const HOST = "http://192.168.1.100:3030";

const post = async (command) =>
  fetch(`${HOST}/${command}`, {
    method: "POST",
    mode: "no-cors",
  });

const Home = () => {
  const [msg, setMsg] = useState("stopped");
  const [isPlaying, setIsPlaying] = useState(false);
  const prevTrack = () => {
    setMsg("prev");
    return post("prev");
  };
  const nextTrack = () => {
    setMsg("next");
    return post("next");
  };
  return (
    <div className="window-body">
      <p style={{ textAlign: "center" }}>{msg}</p>
      <div className="field-row" style={{ justifyContent: "center" }}>
        <button onClick={() => prevTrack()}>⏪</button>
        <button
          onClick={() => {
            setIsPlaying(!isPlaying);
            setMsg(!isPlaying ? "playing" : "stopped");
            return post(!isPlaying ? "play" : "stop");
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
