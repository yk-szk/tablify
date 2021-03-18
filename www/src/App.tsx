import React, { useState, useEffect } from 'react';
import ReactHtmlParser from 'react-html-parser';
import { compressToEncodedURIComponent, decompressFromEncodedURIComponent } from 'lz-string';
import './App.css';
import { toClipboard, downloadAsHtml } from './Utils';

type ModuleType = typeof import('tablify');
let default_template = `<style>
tr:nth-child(odd) {
    background: #f7f7ff;
}
</style>
<table>
{%- if headers %}
<thead>
    <tr>
        {%- for e in headers %}
        <th>{{e}}</th>
        {%- endfor %}
    </tr>
</thead>
{%- endif %}
<tbody>
    {%- for row in rows %}
    <tr>
        {%- for e in row %}
        <td>{{e}}</td>
        {%- endfor %}
    </tr>
    {%- endfor %}
</tbody>
</table>`;

function App() {
  const [wasm, setWasm] = useState<ModuleType | null>(null);
  const [hasHeaders, setHasHeaders] = useState(false);
  const [inputContent, setInputContent] = useState("");
  const [template, setTemplate] = useState(default_template);
  const [output, setOutput] = useState("");
  useEffect(() => {
    const param = new URL(window.location.href).searchParams;
    if (param.has("t")) {
      console.log("Set template from the URL")
      const decoded = decompressFromEncodedURIComponent(param.get("t") as string);
      if (decoded !== null) {
        default_template = decoded;
        setTemplate(default_template);
      }
    }
  }, [])
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
            const table_template = "<table>{%- if headers %}<thead><tr>{%- for e in headers %}<th>{{e}}</th>{%- endfor %}</tr></thead>{%- endif %}<tbody>{%- for row in rows %}<tr>{%- for e in row %}<td>{{e}}</td>{%- endfor %}</tr>{%- endfor %}</tbody></table>";
            const rendered = wasm.render(table_template, new Uint8Array(buf), files[0].name, hasHeaders, false);
            setInputContent(rendered);
            const html = wasm.render(template, new Uint8Array(buf), files[0].name, hasHeaders, false);
            setOutput(html);
          } catch (error) {
            console.error(error);
            setInputContent('<p class="error">' + error + '</p>');
            setOutput("")
          }
        }
      }
    }
  }

  function onTemplateChange(event: React.ChangeEvent<HTMLTextAreaElement>) {
    setTemplate(event.target.value);
    if (event.target.value.length > 0) {
      const compressed = compressToEncodedURIComponent(event.target.value);
      const new_uri = (window.location.pathname.split('/').pop() + '?t=' + compressed);
      window.history.replaceState({}, '', new_uri);
    } else {
      window.history.replaceState({}, '', window.location.pathname);
    }
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
        <input type="checkbox" id="hasHeaders" onChange={(e: any) => setHasHeaders(e.target.checked)} checked={hasHeaders}></input>
        <label htmlFor="hasHeaders">data has headers</label>
      </div>
      <div>
        <h2>Input contents</h2>
        {ReactHtmlParser(inputContent)}
      </div>
      <div>
        <h2>Output</h2>
        <textarea className="code" rows={5} id="output" value={output} readOnly></textarea>
        <button title="Copy output to the clipboard"
          disabled={output === ''}
          onClick={(event) => toClipboard(output)}>Copy</button>
        <button title="Save output as a file"
          disabled={output === ''}
          onClick={(event) => downloadAsHtml(output, "table.html")}>Save</button>
      </div>
    </div>
  );
}

export default App;
