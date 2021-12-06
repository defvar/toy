import * as React from "react";
import { HashRouter as Router, Route, Switch } from "react-router-dom";
import AppDrawer from "./container/AppDrawer";
import { Hello } from "./components/Hello";
import { GraphEdit } from "./container/GraphEdit";
import { Graphs } from "./container/Graphs";
import { Login } from "./container/Login";
import { CssBaseline } from "@mui/material";
import {
    StyledEngineProvider,
    createTheme,
    ThemeProvider,
} from "@mui/material/styles";
import { AuthProvider, AppContextProvider } from "./context";
import { Auth } from "./Auth";
import { SignOut } from "./container/SignOut";

const theme = createTheme({
    palette: {
        mode: "dark",
    },
});

const App = (): JSX.Element => {
    return (
        <AuthProvider>
            <AppContextProvider>
                <StyledEngineProvider injectFirst>
                    <ThemeProvider theme={theme}>
                        <Router>
                            <CssBaseline />
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
                                        <Route path="/signout" exact>
                                            <SignOut />
                                        </Route>
                                    </Switch>
                                </AppDrawer>
                            </Auth>
                            <Route path="/login" exact>
                                <Login redirectTo="/" />
                            </Route>
                        </Router>
                    </ThemeProvider>
                </StyledEngineProvider>
            </AppContextProvider>
        </AuthProvider>
    );
};

export default App;
