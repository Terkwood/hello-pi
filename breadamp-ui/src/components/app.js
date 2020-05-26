import { h, Component } from "preact";
import { Router } from "preact-router";

import Header from "./header";

// Code-splitting is automated for routes
import Home from "../routes/home";
import Songs from "../routes/songs";
import "98.css";

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
      <div id="app" style={{ width: 300 }} className="window">
        <Header />
        <Router onChange={this.handleRoute}>
          <Home path="/" />
          <Songs path="/songs/" />
        </Router>
      </div>
    );
  }
}
