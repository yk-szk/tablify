import React, { useState, useEffect } from 'react';
import init, { render } from 'tablify';
import parse from 'html-react-parser';
import { compressToEncodedURIComponent, decompressFromEncodedURIComponent } from 'lz-string';
import './App.css';
import { toClipboard, downloadAsHtml } from './Utils';

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


function InputTable({ tableHtml }: { tableHtml: string }) {
  if (tableHtml !== "") {
    if (tableHtml.length < 1024) {
      return <>{parse(tableHtml)}</>;
    } else {
      return <details><summary>Show table</summary>{parse(tableHtml)}</details>;
    }
  } else {
    return <></>;
  }
}

type InputData = {
  bytes: Uint8Array;
  name: string;
}

function App() {
  const [wasm, setWasm] = useState<boolean>(false);
  const [inputData, setInputData] = useState<InputData | null>(null);
  const [hasHeaders, setHasHeaders] = useState(false);
  const [ioContents, setIoContents] = useState({ input: "", output: "" });
  const [template, setTemplate] = useState(default_template);
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
      init().then(() => {
        setWasm(true);
        console.log('wasm has been loaded successfully.')
      });
    } catch (err) {
      console.error(`Unexpected error in loading wasm. [Message: ${err}]`);
    }
  }, [])
  useEffect(() => {
    if (wasm && inputData !== null) {
      try {
        const table_template = "<table>{%- if headers %}<thead><tr>{%- for e in headers %}<th>{{e}}</th>{%- endfor %}</tr></thead>{%- endif %}<tbody>{%- for row in rows %}<tr>{%- for e in row %}<td>{{e}}</td>{%- endfor %}</tr>{%- endfor %}</tbody></table>";
        const rendered = render(table_template, inputData.bytes, inputData.name, hasHeaders, false);
        const html = render(template, inputData.bytes, inputData.name, hasHeaders, false);
        setIoContents({ input: rendered, output: html });
      } catch (error) {
        console.error(error);
        setIoContents({ input: '<p class="error">' + error + '</p>', output: "" });
      }
    }
  }, [inputData, hasHeaders, template, wasm]);
  function loadFile(files: FileList) {
    console.log(files);
    if (files.length > 0) {
      const reader = new FileReader();
      reader.readAsArrayBuffer(files[0]);
      reader.onload = function () {
        const buf = reader.result as ArrayBuffer;
        setInputData({ bytes: new Uint8Array(buf), name: files[0].name });
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
        <h1><a id="title" href="/">Tablify</a></h1>
      </header>
      <div>
        <h2>Template <span className="info tooltip" data-tooltip="Set Jinja2/Django template"></span><span className="protip tooltip" data-tooltip="Bookmark this page to reuse the edited template in the future. i.e. template is saved in the URL"></span></h2>
        <textarea className="code" rows={10} value={template} onChange={onTemplateChange} name="template"></textarea >
        <h2>Tabular data <span className="info tooltip" data-tooltip="Choose tabular file (.csv or .xlsx)"></span></h2>
        <form id="inputForm">
          <div>
            <input type="checkbox" id="hasHeaders" onChange={(e: React.ChangeEvent<HTMLInputElement>) => setHasHeaders(e.target.checked)} checked={hasHeaders}></input>
            <label htmlFor="hasHeaders"> check if the data has a header</label>
          </div>
          <input type="file" id="inputFile" accept=".xlsx,.csv" onChange={(e: React.ChangeEvent<HTMLInputElement>) => {
            const input = e.target as HTMLInputElement;
            if (input.files === null) {
              return;
            }
            loadFile(input.files);
          }}></input>
        </form>
        <InputTable tableHtml={ioContents.input} />
      </div>
      <div>
        <h2>Output</h2>
        <textarea className="code" rows={5} id="output" value={ioContents.output} readOnly></textarea>
        <iframe srcDoc={ioContents.output}></iframe>
        <div id="buttons">
          <button title="Copy output to the clipboard"
            disabled={ioContents.output === ''}
            onClick={() => toClipboard(ioContents.output)}>Copy</button>
          <button title="Save output as a file"
            disabled={ioContents.output === ''}
            onClick={() => downloadAsHtml(ioContents.output, "table.html")}>Save</button>
          <input type="reset" form="inputForm" disabled={ioContents.output === ''} value="Clear" title="Clear input and output" onClick={() => {
            setIoContents({ input: "", output: "" });
            const form = document.getElementById("inputForm") as HTMLFormElement;
            form.reset();
            }}></input>
        </div>
      </div>
    </div>
  );
}

export default App
