import * as React from "react";
import { HashRouter as Router, Route, Switch } from "react-router-dom";
import AppDrawer from "./container/AppDrawer";
import { Hello } from "./components/Hello";
import { GraphEdit } from "./container/GraphEdit";
import { Graphs } from "./container/Graphs";
import { Login } from "./container/Login";
import { CssBaseline, createMuiTheme } from "@material-ui/core";
import { ThemeProvider } from "@material-ui/styles";
import { AuthProvider, AppContextProvider } from "./context";
import { Auth } from "./Auth";

const theme = createMuiTheme({
    palette: {
        type: "dark",
        primary: {
            main: "#0097a7",
        },
        secondary: {
            main: "#f50057",
        },
        text: {
            primary: "rgba(255,255,255,1)",
            secondary: "rgba(255,255,255,0.7)",
        },
    },
});

const App = (): JSX.Element => {
    return (
        <AuthProvider>
            <AppContextProvider>
                <ThemeProvider theme={theme}>
                    <Router>
                        <CssBaseline />
                        <Route path="/login" exact>
                            <Login redirectTo="/" />
                        </Route>
                        <Auth redirectByReject="/login">
                            <AppDrawer>
                                <Switch>
                                    <Route path="/" exact>
                                        <Hello
                                            compiler="TypeScript"
                                            framework="React"
                                        />
                                    </Route>
                                    <Route path="/graphs" exact>
                                        <Graphs />
                                    </Route>
                                    <Route path="/graphs/:name/edit" exact>
                                        <GraphEdit />
                                    </Route>
                                </Switch>
                            </AppDrawer>
                        </Auth>
                    </Router>
                </ThemeProvider>
            </AppContextProvider>
        </AuthProvider>
    );
};

export default App;
