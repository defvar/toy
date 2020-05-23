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
import IconButton from "@material-ui/core/IconButton";
import {
    Sidebar,
    Chart,
    ChartData,
    initialChartData,
    toChartData,
} from "./chart";
import ZoomInIcon from "@material-ui/icons/ZoomIn";
import ZoomOutIcon from "@material-ui/icons/ZoomOut";
import CircularProgress from "@material-ui/core/CircularProgress";
import { toResource, Resource } from "../../modules/common";
import {
    GraphEditState,
    ServiceState,
    GraphState,
    Actions,
    reducer,
    initialState,
} from "../../modules/graphEdit";
import { Resizable } from "react-resizable";
import AppBar from "@material-ui/core/AppBar";
import Tabs from "@material-ui/core/Tabs";
import Tab from "@material-ui/core/Tab";
import { Form, Field, addErrors } from "../../components/form";
import { ToyApi } from "../../modules/api/toy-api";

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

const serviceData = {
    services: {
        "common.file.reader": {
            name: "reader",
            namespace: "common.file",
            fullName: "common.file.reader",
            description: "file read service.",
            inPort: 1,
            outPort: 1,
            configSchema: {},
        },
        "common.file.writer": {
            name: "writer",
            namespace: "common.file",
            fullName: "common.file.writer",
            description: "file writer service.",
            inPort: 1,
            outPort: 1,
            configSchema: {},
        },
        "common.map.typed": {
            name: "typed",
            namespace: "common.map",
            fullName: "common..map.typed",
            description: "aaaaaaaaaaaaaaaaa.",
            inPort: 1,
            outPort: 1,
            configSchema: {},
        },
        "common.map.reorder": {
            name: "reorder",
            namespace: "common.map",
            fullName: "common.map.reorder",
            description: "bbbbbbbbbbbbbbb.",
            inPort: 1,
            outPort: 1,
            configSchema: {},
        },
        "aiueo.ccc": {
            name: "ccc",
            namespace: "aiueo",
            fullName: "aiueo.ccc",
            description: "cccccccccccccccccccc.",
            inPort: 1,
            outPort: 1,
            configSchema: {},
        },
    },
    namespaces: {
        "common.file": ["common.file.reader", "common.file.writer"],
        "common.map": ["common.map.typed", "common.map.reorder"],
        aiueo: ["aiueo.ccc"],
    },
};

const graphData = (name: string) => ({
    nodes: {
        reader1: {
            uri: "reader1",
            fullName: "common.file.reader",
            name: "reader",
            namespace: "common.file",
            description: `${name}`,
            inPort: 1,
            outPort: 1,
            position: {
                x: 300,
                y: 100,
            },
            wires: ["writer1"],
        },
        writer1: {
            uri: "writer1",
            fullName: "common.file.writer",
            name: "writer",
            namespace: "common.file",
            description: `${name}`,
            inPort: 1,
            outPort: 1,
            position: {
                x: 300,
                y: 300,
            },
            wires: [],
        },
    },
});

const formData = {
    a1: "aaa",
    a2: "uuuuu",
    a3: 123,
    a4: { a41: true, a42: "dddddd" },
    a5: "aaa",
};

const fetchServices = () => {
    const promise = new Promise<{
        services: { [fullName: string]: ServiceState };
        namespaces: { [namespace: string]: string[] };
    }>((resolve) => {
        setTimeout(() => {
            resolve(serviceData);
        }, 2000);
    });
    return toResource(async () => {
        const r = await ToyApi.getServices();
        console.log(r);
        return await promise;
    });
};

const fetchGraph = (name: string) => {
    const promise = new Promise<GraphState>((resolve) => {
        setTimeout(() => {
            resolve(graphData(name));
        }, 3000);
    });
    return toResource(async () => await promise);
};

interface GraphEditSuspenseProps {
    state: GraphEditState;
    chartState: ChartData;
    setChartState: React.Dispatch<React.SetStateAction<ChartData>>;
    dispatch: React.Dispatch<Actions>;
    serviceResource: Resource<{
        services: { [fullName: string]: ServiceState };
        namespaces: { [namespace: string]: string[] };
    }>;
    graphResource: Resource<GraphState>;
}

const SidebarSuspense = (props: GraphEditSuspenseProps) => {
    const { services, namespaces } = props.serviceResource.read();
    React.useEffect(() => {
        props.dispatch({
            type: "GetServices",
            payload: { services, namespaces },
        });
    }, [services, namespaces]);
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
            payload: { graph },
        });
        const d = toChartData(graph);
        props.setChartState(d);
    }, [graph]);

    return <Chart data={props.chartState} dispatch={props.setChartState} />;
};

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

    const [chartState, setChartState] = React.useState(initialChartData);

    const onChartRefleshClick = React.useCallback(() => {
        setGraphResource(() => fetchGraph(name));
    }, []);

    const onSidebarRefleshClick = React.useCallback(() => {
        setServiceResource(() => fetchServices());
    }, []);

    const handleZoomIn = () => {
        setChartState((prev) => ({
            ...prev,
            scale: prev.scale + 0.1,
        }));
    };

    const handleZoomOut = () => {
        setChartState((prev) => ({
            ...prev,
            scale: prev.scale - 0.1,
        }));
    };

    const [size, setSize] = React.useState(() => {
        return { width: theme.breakpoints.width("md") };
    });
    const onResize = (_event, { size }) => {
        setSize({ width: size.width });
    };

    const [tabNumber, setTabNumber] = React.useState(0);

    const handleTabChange = (
        _event: React.ChangeEvent<{}>,
        newValue: number
    ) => {
        setTabNumber(newValue);
    };

    const [formDataState, setFormDataState] = React.useState(formData);

    const handleFormOnChange = (v) => {
        setFormDataState((prev) => ({
            ...prev,
            ...v,
        }));
    };

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
                                chartState={chartState}
                                setChartState={setChartState}
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
                            chartState={chartState}
                            setChartState={setChartState}
                            dispatch={dispatch}
                            serviceResource={serviceResource}
                            graphResource={graphResource}
                        />
                    </React.Suspense>
                </div>
                <div hidden={tabNumber !== 1}>
                    <Form
                        data={formDataState}
                        liveValidation={true}
                        onChange={handleFormOnChange}
                        validate={(v) => {
                            let r = { name: "root", errors: [] };
                            if (!v.a2 || v.a2 == "") {
                                r = addErrors(r, "a2", [{ message: "reqreq" }]);
                            }
                            r = addErrors(r, "a4.a42", [
                                { message: "aaaaa" },
                                { message: "bbbbbbb" },
                            ]);
                            return r;
                        }}
                        items={
                            [
                                {
                                    name: "a1",
                                    type: "string",
                                    label: "a1",
                                    required: false,
                                },
                                {
                                    name: "a2",
                                    type: "string",
                                    label: "a2",
                                    required: true,
                                },
                                {
                                    name: "a3",
                                    type: "number",
                                    label: "a3",
                                    required: false,
                                },
                                {
                                    name: "a4",
                                    type: "object",
                                    label: "a4",
                                    required: false,
                                    children: [
                                        {
                                            name: "a41",
                                            type: "boolean",
                                            label: "a4-1",
                                            required: false,
                                        },
                                        {
                                            name: "a42",
                                            type: "string",
                                            label: "a4-2",
                                            required: false,
                                        },
                                    ],
                                },
                                {
                                    name: "a5",
                                    type: "enum",
                                    label: "a5",
                                    required: false,
                                    selectOptions: [
                                        { label: "aaa", value: "aaa" },
                                        { label: "bbb", value: "bbb" },
                                        { label: "ccc", value: "ccc" },
                                    ],
                                },
                            ] as Field[]
                        }
                    />
                </div>
            </div>
        </div>
    );
};

export default GraphEdit;
