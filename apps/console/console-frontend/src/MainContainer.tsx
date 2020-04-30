import { hot } from "react-hot-loader";
import * as React from "react";

import { Hello } from "./components/Hello";

const MainContainer = () => (
    <div>
      <Hello compiler="TypeScript" framework="React" />
    </div>
  );

export default hot(module)(MainContainer);
