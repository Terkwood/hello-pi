import { h, Component } from "preact";
//import { useState } from "preact/hooks";
import { Router } from "preact-router";

import Header from "./header";

// Code-splitting is automated for routes
import Home from "../routes/home";
import Profile from "../routes/profile";

//const [count, setCount] = useState(0);
const count = 0;
const setCount = (_no) => {
  return;
};
const Proto = () => {
  return (<div
    style={{
      height: "100vh",
      display: "flex",
      justifyContent: "center",
      alignItems: "center",
    }}
  >
    <div style={{ width: 350 }}>
      <div style={{ width: 300 }} className="window">
        <div className="title-bar">
          <div className="title-bar-text">Counter</div>
          <div className="title-bar-controls">
            <button aria-label="Minimize" />
            <button aria-label="Maximize" />
            <button aria-label="Close" />
          </div>
        </div>

        <div className="window-body">
          <p style={{ textAlign: "center" }}>Current count: {count}</p>
          <div className="field-row" style={{ justifyContent: "center" }}>
            <button onClick={() => setCount(count + 1)}>+</button>
            <button onClick={() => setCount(count - 1)}>-</button>
            <button onClick={() => setCount(0)}>0</button>
          </div>
        </div>
      </div>
    </div>
  </div>);
};

export default class App extends Component {
  /** Gets fired when the route changes.
	 *	@param {Object} event		"change" event from [preact-router](http://git.io/preact-router)
	 *	@param {string} event.url	The newly routed URL
	 */
  handleRoute = (event) => {
    this.currentUrl = event.url;
  };

  render() {
    return (
      <div id="app"><Proto /></div>
    );
  }
}

//render(<App />, document.body);
