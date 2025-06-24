# ts-scss-modules

<!-- Neo Brutalism Style -->
<style>
.neo-brutalism {
  background: #fff;
  border: 4px solid #000;
  border-radius: 12px;
  box-shadow: 8px 8px 0 #000;
  padding: 2em;
  margin: 2em 0;
  font-family: 'Fira Mono', 'Consolas', monospace;
}
.neo-brutalism h2 {
  background: #2196f3;
  background: linear-gradient(90deg, #2196f3 0%, #21cbf3 100%);
  color: #fff;
  padding: 0.5em 1.5em;
  border-radius: 10px;
  display: inline-block;
  box-shadow: 4px 4px 0 #000;
  margin-bottom: 1em;
}
.neo-brutalism-title {
  background: #fff;
  border: 4px solid #000;
  border-radius: 18px;
  box-shadow: 10px 10px 0 #000;
  padding: 2.5em 2em 2em 2em;
  margin: 2em 0 2em 0;
  text-align: center;
}
.neo-brutalism-title h1 {
  background: linear-gradient(90deg, #2196f3 0%, #21cbf3 100%);
  color: #fff;
  padding: 0.7em 2em;
  border-radius: 12px;
  display: inline-block;
  box-shadow: 6px 6px 0 #000;
  font-size: 2.5em;
  margin-bottom: 0.5em;
}
.neo-brutalism-title p {
  color: #222;
  font-size: 1.2em;
  margin-top: 1em;
}
.neo-brutalism pre {
  background: #e3f2fd;
  color: #000;
  border: 2px solid #000;
  border-radius: 6px;
  padding: 1em;
  box-shadow: 2px 2px 0 #000;
}
.neo-brutalism ul {
  background: #f5f5f5;
  border: 2px solid #000;
  border-radius: 6px;
  padding: 1em 2em;
  box-shadow: 2px 2px 0 #000;
}
</style>

<div class="neo-brutalism-title">
  <h1>ts-scss-modules</h1>
  <p>⚡️ Generate TypeScript definitions for your SCSS modules with blazing-fast Rust CLI.</p>
</div>

<div class="neo-brutalism">
  <h2>Usage</h2>
  <ul>
    <li>Install <b>Rust</b> if you haven't: <a href="https://rustup.rs/">https://rustup.rs/</a></li>
    <li>Clone this repository and <code>cd</code> into it.</li>
    <li>Run the CLI tool with your project path:
      <pre><code>cargo run -- --path ./your-ts-project</code></pre>
    </li>
    <li>To specify the number of threads (default: 4):
      <pre><code>cargo run -- --threads 8 --path ./your-ts-project</code></pre>
    </li>
    <li>Set log level for more output:
      <pre><code>cargo run -- --log-level debug --path ./your-ts-project</code></pre>
    </li>
  </ul>
</div>

<div class="neo-brutalism">
  <h2>What does it do?</h2>
  <ul>
    <li>Recursively scans your project for <code>.scss</code> files (excluding <code>node_modules</code>, <code>dist</code>, <code>.yalc</code>, and hidden folders).</li>
    <li>Parses each SCSS file to extract all class names (including nested classes).</li>
    <li>Generates a <code>.scss.d.ts</code> TypeScript declaration file next to each SCSS file, exporting all class names as a type-safe object.</li>
    <li>Uses multi-threading for fast processing of large codebases.</li>
  </ul>
</div>

<div class="neo-brutalism">
  <h2>CLI Options</h2>
  <ul>
    <li><code>--path &lt;dir&gt;</code> (required): Root directory to scan for SCSS files.</li>
    <li><code>--threads &lt;N&gt;</code> (optional): Number of parallel threads (default: 4).</li>
    <li><code>--log-level &lt;level&gt;</code> (optional): Set log level (<code>info</code>, <code>debug</code>, etc.).</li>
  </ul>
</div>

<div class="neo-brutalism">
  <h2>Example Output</h2>
  <pre><code>export type Styles = {
  myClass: string;
  anotherClass: string;
}

export type ClassNames = keyof Styles;
declare const styles: Styles;
export default styles;
</code></pre>
</div>

<div class="neo-brutalism">
  <h2>Roadmap</h2>
  <ul>
    <li><s>Basic parsing <b>(done)</b></s></li>
    <li>Full documentation <b>(in progress)</b></li>
    <li>Custom templates <b>(planned)</b></li>
    <li>Binary download <b>(planned)</b></li>
    <li>Installation via cargo <b>(planned)</b></li>
    <li>Performance on bigger scss files <b>(planned)</b></li>
  </ul>
</div>

<div class="neo-brutalism" style="text-align:center;">
  <p style="font-size:1.3em; font-weight:bold; margin-bottom:0.5em;">BUILT WITH 💪 RUST FOR TYPESCRIPT</p>
  <p style="font-size:1.1em; margin-bottom:0.5em;">MIT License - Generate types, not problems</p>
</div>
