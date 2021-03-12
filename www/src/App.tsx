import React, { useState, useEffect } from 'react';
import './App.css';

type ModuleType = typeof import('tablify');

function App() {
  // const [wasm, setWasm] = useState<ModuleType>();
  useEffect(() => {
    try {
      import('tablify').then((module: ModuleType) => {
        const arr = new Uint8Array(10);
        try {
            const html = module.render("<p>template</p>", arr, "tmp.xlsx");
        } catch(error) {
            console.error(error);
        }
                // setWasm(module);
        console.log('Yey!')
      });
    } catch (err) {
      console.error(`Unexpected error in loading wasm. [Message: ${err.message}]`);
    }
  }, [])
  return (
    <div className="App">
      <p>hello</p>
    </div>
  );
}

export default App;
