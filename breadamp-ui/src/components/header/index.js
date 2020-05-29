import * as preact from "preact";
import { Link } from "preact-router/match";

const Header = () => (
  <header className="title-bar">
    <div className="title-bar-text">BreadAmp 🍞</div>
    <nav>
      <div className="title-bar-controls">
        <Link href="/">
          <button aria-label="Minimize" />
        </Link>
        <Link href="/songs">
          <button aria-label="Maximize" />
        </Link>
        <button aria-label="Close" />
      </div>
    </nav>
  </header>
);

export default Header;
