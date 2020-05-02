import { hot } from "react-hot-loader";
import * as React from "react";
import {
  HashRouter as Router,
  Route,
  Switch,
} from 'react-router-dom';
import { createStyles, Theme, makeStyles } from '@material-ui/core/styles';
import { Hello } from "./components/Hello";
import { SideBar } from "./components/SideBar";
import { GraphEdit } from "./container/GraphEdit";
import { Graphs } from "./container/Graphs";

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      display: 'flex',
    },
    content: {
      flexGrow: 1,
      backgroundColor: theme.palette.background.default,
      padding: theme.spacing(3),
    },
  }),
);

const MainContainer = () => {
  const classes = useStyles();
  return (
    <div className={classes.root} >
      <Router>
        <SideBar />
        <main className={classes.content}>
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
        </main>
      </Router>
    </div>
  )
};

export default hot(module)(MainContainer);
