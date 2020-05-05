import * as React from "react";
import {
  HashRouter as Router,
  Route,
  Switch,
} from 'react-router-dom';
import CircularProgress from '@material-ui/core/CircularProgress';
import AppDrawer from './container/AppDrawer';

const Hello = React.lazy(() => import("./components/Hello"));
const GraphEdit = React.lazy(() => import("./container/GraphEdit"));
const Graphs = React.lazy(() => import("./container/Graphs"));

const Loading = () => <CircularProgress />;

const App = () => {
  return (
    <Router>
      <AppDrawer>
        <React.Suspense fallback={Loading()}>
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
        </React.Suspense>
      </AppDrawer>
    </Router>
  )
};

export default App;
