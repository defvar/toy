import * as React from "react";
import { useParams } from "react-router-dom";
import { styled } from "@mui/material/styles";
import RefreshIcon from "@mui/icons-material/Refresh";
import AddIcon from "@mui/icons-material/Add";
import {
    Box,
    Tab,
    Tabs,
    IconButton,
    Paper,
    Stack,
    Typography,
    Divider,
} from "@mui/material";
import TabList from "@mui/lab/TabList";
import TabContext from "@mui/lab/TabContext";
import TabPanel from "@mui/lab/TabPanel";
import { Chart } from "./chart";
import { reducer, initialState } from "../../modules/graphEdit";
import { fetchGraph } from "../../modules/api/toy-api";
import CircularProgress from "../../components/progress/CircularProgress";
import { Resizable } from "react-resizable";
import { NodeEditor } from "./NodeEditor";
import ServiceSelector from "./ServiceList";

const ResizeHandle = styled("span")(({ theme }) => ({
    position: "absolute",
    width: "2px",
    height: "100%",
    backgroundColor: theme.palette.divider,
    opacity: "0.75",
    top: "0",
    cursor: "ew-resize",
}));

const ResizeHandleBottom = styled("span")(({ theme }) => ({
    position: "absolute",
    width: "100%",
    height: "2px",
    backgroundColor: theme.palette.divider,
    opacity: "0.75",
    bottom: "0",
    cursor: "ns-resize",
    zIndex: 11000,
}));

const LeftPane = styled(Box)(({ theme }) => ({
    flexGrow: 1,
    zIndex: 990,
    marginRight: theme.spacing(3),
    height: "100%",
}));

const OuterResizable = styled(Resizable)(({ theme }) => ({
    position: "relative",
    display: "flex",
    flexGrow: 1,
}));

const RightResizable = styled(Resizable)(({ theme }) => ({
    position: "relative",
}));

const BottomPane = styled(Box)(({ theme }) => ({
    bottom: 0,
    left: 0,
    width: "100%",
    backgroundColor: theme.palette.background.default,
    zIndex: 10000,
}));

export const GraphEdit = () => {
    const { name } = useParams<{ name: string }>();
    const [graphResource, setGraphResource] = React.useState(() =>
        fetchGraph(name)
    );
    const [state, dispatch] = React.useReducer(reducer, initialState);
    const [tabNumber, setTabNumber] = React.useState(0);
    const [bottomTabNumber, setBottomTabNumber] = React.useState("0");
    const [serviceListOpen, setServiceListOpen] = React.useState(false);

    const onChartRefleshClick = React.useCallback(() => {
        setGraphResource(() => fetchGraph(name));
    }, []);

    const onAddClick = React.useCallback(() => {
        setServiceListOpen(true);
    }, []);

    const onServiceListClose = React.useCallback(() => {
        setServiceListOpen(false);
    }, []);

    const [rightPaneSize, setRightPaneSize] = React.useState(() => {
        return { width: 240 };
    });

    const [contentSize, setContentSize] = React.useState(() => {
        return { content: 500, bottom: 250 };
    });

    const onRightPaneResize = (_event, { size }) => {
        setRightPaneSize({ width: size.width });
    };

    const onBottomResize = (_event, { size }) => {
        setContentSize((prev) => {
            const delta = prev.content - size.height;
            return {
                content: size.height,
                bottom: prev.bottom + delta,
            };
        });
    };

    const onBottomTabChange = React.useCallback(
        (_event: React.ChangeEvent<{}>, newValue: string) => {
            setBottomTabNumber(newValue);
        },
        []
    );

    return (
        <Stack spacing={0}>
            <OuterResizable
                width={Infinity}
                height={contentSize.content}
                onResize={onBottomResize}
                handle={<ResizeHandleBottom />}
                resizeHandles={["s"]}
            >
                <Box
                    sx={{
                        display: "flex",
                        height: contentSize.content,
                        zIndex: 1200,
                    }}
                >
                    <LeftPane>
                        <Stack spacing={0} sx={{ height: "100%" }}>
                            <Paper sx={{ height: "100%", width: "100%", p: 1 }}>
                                <Box sx={{ paddingLeft: 1, paddingBottom: 1 }}>
                                    <Typography variant="h6">{name}</Typography>
                                </Box>
                                <Divider />
                                <Box sx={{ width: "100%" }}>
                                    <IconButton
                                        aria-label="refresh"
                                        onClick={onChartRefleshClick}
                                        size="large"
                                    >
                                        <RefreshIcon />
                                    </IconButton>
                                    <IconButton
                                        aria-label="add"
                                        onClick={onAddClick}
                                        size="large"
                                    >
                                        <AddIcon />
                                    </IconButton>
                                </Box>
                                <Divider />
                                <Box
                                    p={2}
                                    sx={{ height: "100%", display: "flex" }}
                                >
                                    <React.Suspense
                                        fallback={<CircularProgress />}
                                    >
                                        <Chart
                                            data={state.chart}
                                            graphResource={graphResource}
                                            dispatch={dispatch}
                                        />
                                    </React.Suspense>
                                    <ServiceSelector
                                        open={serviceListOpen}
                                        onClose={onServiceListClose}
                                    />
                                </Box>
                            </Paper>
                        </Stack>
                    </LeftPane>
                    <RightResizable
                        width={rightPaneSize.width}
                        height={Infinity}
                        onResize={onRightPaneResize}
                        handle={<ResizeHandle />}
                        resizeHandles={["w"]}
                    >
                        <Box
                            sx={{
                                width: rightPaneSize.width,
                                height: "100%",
                                overflowY: "auto",
                            }}
                        >
                            <Tabs
                                value={tabNumber}
                                aria-label="tabs"
                                variant="scrollable"
                                scrollButtons="auto"
                            >
                                <Tab label="config" />
                            </Tabs>
                            <NodeEditor state={state} dispatch={dispatch} />
                        </Box>
                    </RightResizable>
                </Box>
            </OuterResizable>
            <BottomPane sx={{ height: contentSize.bottom }}>
                <TabContext value={bottomTabNumber}>
                    <Paper
                        variant="outlined"
                        sx={{ height: "100%", width: "100%", p: 1 }}
                    >
                        <TabList
                            value={bottomTabNumber}
                            onChange={onBottomTabChange}
                            aria-label="tabs"
                            variant="scrollable"
                            scrollButtons="auto"
                        >
                            <Tab label="CONSOLE" value="0" />
                            <Tab label="CODE" value="1" />
                        </TabList>
                        <TabPanel value="0"></TabPanel>
                        <TabPanel value="1">
                            <Box>{JSON.stringify(state.nodes, null, 2)}</Box>
                        </TabPanel>
                    </Paper>
                </TabContext>
            </BottomPane>
        </Stack>
    );
};

export default GraphEdit;
