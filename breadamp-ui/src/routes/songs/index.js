import { h, Component } from "preact";
import "98.css";

export default class Songs extends Component {
  state = {
    time: Date.now(),
    count: 10,
  };

  // update the current time
  updateTime = () => {
    this.setState({ time: Date.now() });
  };

  increment = () => {
    this.setState({ count: this.state.count + 1 });
  };

  // gets called when this route is navigated to
  componentDidMount() {
    // start a timer for the clock:
    this.timer = setInterval(this.updateTime, 1000);
  }

  // gets called just before navigating away from the route
  componentWillUnmount() {
    clearInterval(this.timer);
  }

  render({ user }, { time, count }) {
    return (
      <div className="window-body">
        <strong>Songs</strong>
        <p>This is a long list of songs.</p>

        <div>Current time: {new Date(time).toLocaleString()}</div>

        <p>
          <button onClick={this.increment}>Click Me</button> Clicked {count}
          {" "}
          times.
        </p>
      </div>
    );
  }
}
