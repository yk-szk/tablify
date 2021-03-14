import React, { useState, useEffect } from 'react';
import ReactHtmlParser from 'react-html-parser';
import './App.css';
import { toClipboard } from './Utils';

type ModuleType = typeof import('tablify');
const default_template = `<style>
    p {
        text-indent: 1em;
        padding: 0;
        margin-bottom: 0 !important;
    }

    address {
        font-style: normal;
        margin: 1em 0 !important;
    }
</style>
<table>
    <tbody>
        {%- for row in rows %}
        <tr>
            <th align="center" width="30%">
                {{row[0]}}
            </th>
            <td width="70%">
                {{row[1]}}
            </td>
        </tr>
        {%- endfor %}
    </tbody>
</table>`;

function App() {
  const [wasm, setWasm] = useState<ModuleType | null>(null);
  const [tableRows, setTableRows] = useState("");
  const [template, setTemplate] = useState(default_template);
  const [output, setOutput] = useState("");
  useEffect(() => {
    try {
      import('tablify').then((module: ModuleType) => {
        setWasm(module);
        console.log('wasm has been loaded successfully.')
      });
    } catch (err) {
      console.error(`Unexpected error in loading wasm. [Message: ${err.message}]`);
    }
  }, [])
  function loadFile(files: FileList) {
    console.log(files);
    if (files.length > 0) {
      const reader = new FileReader();
      reader.readAsArrayBuffer(files[0]);
      reader.onload = function () {
        if (wasm !== null) {
          const buf = reader.result as ArrayBuffer;
          try {
            const table_template = "{%- for row in rows %}<tr>{%- for e in row %}<td>{{e}}</td>{%- endfor %}</tr>{%- endfor %}";
            const rendered = wasm.render(table_template, new Uint8Array(buf), files[0].name, false);
            setTableRows(rendered);
            const html = wasm.render(template, new Uint8Array(buf), files[0].name, false);
            console.log(html);
            setOutput(html);
          } catch (error) {
            console.error(error);
          }
        }
      }
    }
  }

  function onTemplateChange(event: React.ChangeEvent<HTMLTextAreaElement>) {
    setTemplate(event.target.value);
  }

  return (
    <div className="App">
      <header>
        <h1>Tablify</h1>
      </header>
      <div>
        <h2>Template</h2>
        <p className="usage">Set Jinja2/Django template</p>
        <textarea className="code" rows={5} value={template} onChange={onTemplateChange} name="template"></textarea >
        <h2>Tabular data</h2>
        <p className="usage">Choose tabular file (.csv or .xlsx)</p>
        <input type="file" accept=".xlsx,.csv" onChange={(e: any) => loadFile(e.target.files)}></input>
      </div>
      <div>
        <h2>Input contents</h2>
        <table>
          <tbody>
            {ReactHtmlParser(tableRows)}
          </tbody>
        </table>
      </div>
      <div>
        <h2>Output</h2>
        <textarea className="code" rows={5} id="output" value={output} readOnly></textarea>
        <button title="Copy output to the clipboard"
          disabled={output === ''}
          onClick={(event) => toClipboard(output)}>Copy</button>
      </div>
    </div>
  );
}

export default App;
