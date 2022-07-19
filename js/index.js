import React from "react";
import ReactDOM from "react-dom";
import { useWasm } from "../hooks/useWasm.js";

function Main() {
  const wasmModule = useWasm();
  return (
    <>
      <p> Hello, from react!</p>
      {wasmModule && (
        <p> This is value is computed with rust/wasm {wasmModule.add(3, 4)}</p>
      )}
    </>
  );
}

ReactDOM.render(<Main />, document.getElementById("root"));
