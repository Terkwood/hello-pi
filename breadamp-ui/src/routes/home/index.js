import { useState } from "preact/hooks";
import "98.css";

const Home = () => {
  const [count, setCount] = useState(0);
  return (
    <div className="window-body">
      <p style={{ textAlign: "center" }}>Current count: {count}</p>
      <div className="field-row" style={{ justifyContent: "center" }}>
        <button onClick={() => setCount(count + 1)}>⏪</button>
        <button onClick={() => setCount(count - 1)}>▶️</button>
        <button onClick={() => setCount(0)}>⏩</button>
      </div>
    </div>
  );
};

export default Home;
