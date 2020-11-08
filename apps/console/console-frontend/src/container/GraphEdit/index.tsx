import * as React from "react";
import { useParams } from "react-router-dom";
import {
    createStyles,
    Theme,
    makeStyles,
    useTheme,
} from "@material-ui/core/styles";
import Typography from "@material-ui/core/Typography";
import RefreshIcon from "@material-ui/icons/Refresh";
import UndoIcon from "@material-ui/icons/Undo";
import IconButton from "@material-ui/core/IconButton";
import { Sidebar, Chart } from "./chart";
import ZoomInIcon from "@material-ui/icons/ZoomIn";
import ZoomOutIcon from "@material-ui/icons/ZoomOut";
import CircularProgress from "@material-ui/core/CircularProgress";
import { Resource } from "../../modules/common";
import {
    GraphEditState,
    Actions,
    reducer,
    initialState,
} from "../../modules/graphEdit";
import { Resizable } from "react-resizable";
import AppBar from "@material-ui/core/AppBar";
import Tabs from "@material-ui/core/Tabs";
import Tab from "@material-ui/core/Tab";
import { Form, ValidationResult } from "../../components/form";
import {
    ServiceResponse,
    fetchServices,
    fetchGraph,
    GraphResponse,
} from "../../modules/api/toy-api";

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            flexGrow: 1,
            display: "flex",
        },
        heading: {
            fontSize: theme.typography.pxToRem(15),
        },
        leftPane: {
            position: "relative",
        },
        rightPane: {
            marginLeft: theme.spacing(3),
            flexGrow: 1,
            backgroundColor: theme.palette.background.paper,
            zIndex: 990,
        },
        chartCanvas: {
            overflow: "hidden",
            maxHeight: "80vh",
        },
        loader: {
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
        },
        progress: {},
        resizeHandle: {
            position: "absolute",
            width: "4px",
            height: "100px",
            backgroundColor: "#1153aa",
            opacity: "0.75",
            top: "30%",
            marginTop: "-4px",
            right: "-11px",
            cursor: "ew-resize",
        },
    })
);

interface GraphEditSuspenseProps {
    state: GraphEditState;
    dispatch: React.Dispatch<Actions>;
    serviceResource: Resource<ServiceResponse>;
    graphResource: Resource<GraphResponse>;
}

const SidebarSuspense = (props: GraphEditSuspenseProps) => {
    const r = props.serviceResource.read();
    React.useEffect(() => {
        props.dispatch({
            type: "GetServices",
            payload: r,
        });
    }, [r]);
    return (
        <Sidebar
            services={props.state.services}
            namespaces={props.state.namespaces}
        />
    );
};

const ChartSuspense = (props: GraphEditSuspenseProps) => {
    const graph = props.graphResource.read();
    React.useEffect(() => {
        props.dispatch({
            type: "GetGraph",
            payload: graph,
        });
    }, [graph]);

    return <Chart data={props.state.graph} dispatch={props.dispatch} />;
};

function validate(v: any): ValidationResult {
    const r = { name: "root", errors: [] };
    // if (!v.a2 || v.a2 == "") {
    //     r = addErrors(r, "a2", [{ message: "reqreq" }]);
    // }
    // r = addErrors(r, "a4.a42", [{ message: "aaaaa" }, { message: "bbbbbbb" }]);
    return r;
}

export const GraphEdit = () => {
    const { name } = useParams();
    const classes = useStyles();
    const theme = useTheme();

    const [serviceResource, setServiceResource] = React.useState(() =>
        fetchServices()
    );
    const [graphResource, setGraphResource] = React.useState(() =>
        fetchGraph(name)
    );
    const [state, dispatch] = React.useReducer(reducer, initialState);
    const [tabNumber, setTabNumber] = React.useState(0);

    const onChartRefleshClick = React.useCallback(() => {
        setGraphResource(() => fetchGraph(name));
    }, []);

    const onSidebarRefleshClick = React.useCallback(() => {
        setServiceResource(() => fetchServices());
    }, []);

    const onPropertyRevertClick = React.useCallback(() => {
        console.log("revert!");
    }, []);

    const handleZoomIn = React.useCallback(() => {
        dispatch({
            type: "ZoomChart",
            payload: 0.1,
        });
    }, []);

    const handleZoomOut = React.useCallback(() => {
        dispatch({
            type: "ZoomChart",
            payload: -0.1,
        });
    }, []);

    const [size, setSize] = React.useState(() => {
        return { width: theme.breakpoints.width("md") };
    });

    const onResize = (_event, { size }) => {
        setSize({ width: size.width });
    };

    const handleTabChange = React.useCallback(
        (_event: React.ChangeEvent<{}>, newValue: number) => {
            setTabNumber(newValue);
        },
        [state.services]
    );

    const handleFormOnChange = React.useCallback((v) => {
        dispatch({
            type: "ChangeEditNode",
            payload: v,
        });
    }, []);

    return (
        <div className={classes.root}>
            <Resizable
                width={size.width}
                height={Infinity}
                className={classes.leftPane}
                onResize={onResize}
                handle={<span className={classes.resizeHandle} />}
                resizeHandles={["e"]}
            >
                <div
                    style={{
                        width: size.width + "px",
                    }}
                >
                    <Typography className={classes.heading}>{name}</Typography>
                    <IconButton
                        aria-label="refresh"
                        onClick={onChartRefleshClick}
                    >
                        <RefreshIcon />
                    </IconButton>
                    <IconButton aria-label="zoom-in" onClick={handleZoomIn}>
                        <ZoomInIcon />
                    </IconButton>
                    <IconButton aria-label="zoom-out" onClick={handleZoomOut}>
                        <ZoomOutIcon />
                    </IconButton>
                    <div
                        style={{
                            width: size.width + "px",
                        }}
                        className={classes.chartCanvas}
                    >
                        <React.Suspense
                            fallback={
                                <div className={classes.loader}>
                                    <CircularProgress
                                        size={68}
                                        className={classes.progress}
                                    />
                                </div>
                            }
                        >
                            <ChartSuspense
                                state={state}
                                dispatch={dispatch}
                                serviceResource={serviceResource}
                                graphResource={graphResource}
                            />
                        </React.Suspense>
                    </div>
                </div>
            </Resizable>
            <div className={classes.rightPane}>
                <AppBar position={"relative"}>
                    <Tabs
                        value={tabNumber}
                        onChange={handleTabChange}
                        aria-label="tabs"
                    >
                        <Tab label="Services" />
                        <Tab label="Property" />
                    </Tabs>
                </AppBar>
                <div hidden={tabNumber !== 0}>
                    <IconButton
                        aria-label="refresh"
                        onClick={onSidebarRefleshClick}
                    >
                        <RefreshIcon />
                    </IconButton>
                    <React.Suspense
                        fallback={
                            <div className={classes.loader}>
                                <CircularProgress
                                    size={68}
                                    className={classes.progress}
                                />
                            </div>
                        }
                    >
                        <SidebarSuspense
                            state={state}
                            dispatch={dispatch}
                            serviceResource={serviceResource}
                            graphResource={graphResource}
                        />
                    </React.Suspense>
                </div>
                <div hidden={tabNumber !== 1}>
                    <IconButton
                        aria-label="undo"
                        onClick={onPropertyRevertClick}
                    >
                        <UndoIcon />
                    </IconButton>
                    <Typography className={classes.heading}>
                        {state.edit.id}
                    </Typography>
                    <Form
                        data={state.edit.config}
                        liveValidation={false}
                        onChange={handleFormOnChange}
                        validate={validate}
                        schema={state.edit.configSchema}
                    />
                </div>
            </div>
        </div>
    );
};

export default GraphEdit;
