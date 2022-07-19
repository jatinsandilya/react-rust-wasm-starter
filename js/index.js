import("../pkg/index.js")
  .catch(console.error)
  .then((module) => {
    console.log(
      "hello from method defined in Rust but called in JS land",
      module.add(3, 4)
    );
  });

import React from "react";
import ReactDOM from "react-dom";

function Main() {
  return <p> Hello, from react!</p>;
}

ReactDOM.render(<Main />, document.getElementById("root"));
