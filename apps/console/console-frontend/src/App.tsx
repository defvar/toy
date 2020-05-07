import * as React from "react";
import {
  HashRouter as Router,
  Route,
  Switch,
} from 'react-router-dom';
import AppDrawer from './container/AppDrawer';
import { Hello } from "./components/Hello";
import { GraphEdit } from "./container/GraphEdit";
import { Graphs } from "./container/Graphs";

const App = () => {
  return (
    <Router>
      <AppDrawer>
        <Switch>
          <Route path="/" exact>
            <Hello compiler="TypeScript" framework="React" />
          </Route>
          <Route path="/graphs" exact>
            <Graphs />
          </Route>
          <Route path="/graphs/:name/edit" exact>
            <GraphEdit />
          </Route>
        </Switch>
      </AppDrawer>
    </Router>
  )
};

export default App;
