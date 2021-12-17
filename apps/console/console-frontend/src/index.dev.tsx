import * as React from "react";
import * as ReactDOM from "react-dom";
import { hot } from "react-hot-loader/root";

import App from "./App";

const HotContainer = hot(App);

ReactDOM.render(<HotContainer />, document.getElementById("root"));
