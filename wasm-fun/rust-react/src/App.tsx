import React from 'react';
import logo from './logo.svg';
import './App.css';

function App() {

  // We can get away without explicitly specifing a type but then the linter wouldn't know the type of {wasm}
  // This will initialize {wasm} as undefined
  const [wasm, setWasm] = React.useState<any>();

  // Same as component did mount, called when the component is first initialized
  React.useEffect(() => {
    loadWasm();
  }, []);

  // Asyncronously load webassembly
  const loadWasm = async () => {
    try {
      const wasm = await import('@foo/rust-wasm');
      // Update state
      setWasm(wasm);
    } catch(err) {
      console.error('Unexpected error in loadWasm. [Message: ${err.message}]');
    }
  }

  const onLogoClick = () => {
    // Check if wasm is loaded
    console.log(wasm);
    if (wasm) {
      wasm.greet("Hello from WASM!");
    }
  }

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p onClick={onLogoClick}>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
