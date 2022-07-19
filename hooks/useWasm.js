import { useState, useEffect } from "react";

export const useWasm = () => {
  const [wasmModule, setWasmModule] = useState();

  useEffect(() => {
    (async () => {
      try {
        const wasm = await import("../pkg/index.js");
        setWasmModule(wasm);
      } catch (e) {
        console.error(e);
      }
    })();
  }, []);

  return wasmModule;
};
