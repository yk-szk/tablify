(this.webpackJsonptablify=this.webpackJsonptablify||[]).push([[0],{29:function(e,t,n){},54:function(e,t){},73:function(e,t,n){},75:function(e,t,n){"use strict";n.r(t);var a=n(1),r=n.n(a),c=n(23),o=n.n(c),l=(n(29),n(4)),s=n(24),i=n.n(s),d=n(8);n(73);var b=n(0),u="<style>\ntr:nth-child(odd) {\n    background: #f7f7ff;\n}\n</style>\n<table>\n<tbody>\n    {%- for row in rows %}\n    <tr>\n        {%- for e in row %}\n        <td>{{e}}</td>\n        {%- endfor %}\n    </tr>\n    {%- endfor %}\n</tbody>\n</table>";var j=function(){var e=Object(a.useState)(null),t=Object(l.a)(e,2),r=t[0],c=t[1],o=Object(a.useState)(""),s=Object(l.a)(o,2),j=s[0],h=s[1],f=Object(a.useState)(u),p=Object(l.a)(f,2),O=p[0],m=p[1],v=Object(a.useState)(""),x=Object(l.a)(v,2),w=x[0],g=x[1];return Object(a.useEffect)((function(){var e=new URL(window.location.href).searchParams;if(e.has("t")){console.log("Set template from the URL");var t=Object(d.decompressFromEncodedURIComponent)(e.get("t"));null!==t&&m(u=t)}}),[]),Object(a.useEffect)((function(){try{n.e(3).then(n.bind(null,79)).then((function(e){c(e),console.log("wasm has been loaded successfully.")}))}catch(e){console.error("Unexpected error in loading wasm. [Message: ".concat(e.message,"]"))}}),[]),Object(b.jsxs)("div",{className:"App",children:[Object(b.jsx)("header",{children:Object(b.jsx)("h1",{children:"Tablify"})}),Object(b.jsxs)("div",{children:[Object(b.jsx)("h2",{children:"Template"}),Object(b.jsx)("p",{className:"usage",children:"Set Jinja2/Django template"}),Object(b.jsx)("textarea",{className:"code",rows:5,value:O,onChange:function(e){if(m(e.target.value),e.target.value.length>0){var t=Object(d.compressToEncodedURIComponent)(e.target.value),n=window.location.pathname.split("/").pop()+"?t="+t;window.history.replaceState({},"",n)}else window.history.replaceState({},"",window.location.pathname)},name:"template"}),Object(b.jsx)("h2",{children:"Tabular data"}),Object(b.jsx)("p",{className:"usage",children:"Choose tabular file (.csv or .xlsx)"}),Object(b.jsx)("input",{type:"file",accept:".xlsx,.csv",onChange:function(e){return function(e){if(console.log(e),e.length>0){var t=new FileReader;t.readAsArrayBuffer(e[0]),t.onload=function(){if(null!==r){var n=t.result;try{var a=r.render("{%- for row in rows %}<tr>{%- for e in row %}<td>{{e}}</td>{%- endfor %}</tr>{%- endfor %}",new Uint8Array(n),e[0].name,!1);h(a);var c=r.render(O,new Uint8Array(n),e[0].name,!1);console.log(c),g(c)}catch(o){console.error(o)}}}}}(e.target.files)}})]}),Object(b.jsxs)("div",{children:[Object(b.jsx)("h2",{children:"Input contents"}),Object(b.jsx)("table",{children:Object(b.jsx)("tbody",{children:i()(j)})})]}),Object(b.jsxs)("div",{children:[Object(b.jsx)("h2",{children:"Output"}),Object(b.jsx)("textarea",{className:"code",rows:5,id:"output",value:w,readOnly:!0}),Object(b.jsx)("button",{title:"Copy output to the clipboard",disabled:""===w,onClick:function(e){return t=w,void(navigator.clipboard&&navigator.clipboard.writeText(t));var t},children:"Copy"}),Object(b.jsx)("button",{title:"Save output as a file",disabled:""===w,onClick:function(e){return function(e,t){var n=new Blob([e],{type:"text/html"}),a=document.createElement("a");a.href=URL.createObjectURL(n),a.download=t,a.click(),a.remove()}(w,"table.html")},children:"Save"})]})]})};o.a.render(Object(b.jsx)(r.a.StrictMode,{children:Object(b.jsx)(j,{})}),document.getElementById("root"))}},[[75,1,2]]]);
//# sourceMappingURL=main.11cf1dfd.chunk.js.map