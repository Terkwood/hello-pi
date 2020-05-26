import { h } from "preact";
import { Link } from "preact-router/match";

const Header = () => (
  <header style={{ width: 300 }} className="window">
    <div className="title-bar">
      <div className="title-bar-text">Counter</div>
      <nav>
        <div className="title-bar-controls">
          <Link href="/"><button aria-label="Minimize" /></Link>
          <Link href="/songs"><button aria-label="Maximize" /></Link>
          <button aria-label="Close" />
        </div>
      </nav>
    </div>
  </header>
);

export default Header;
