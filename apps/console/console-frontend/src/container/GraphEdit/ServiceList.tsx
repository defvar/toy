import * as React from "react";
import {
    Box,
    Grid,
    Modal,
    Card,
    CardContent,
    Typography,
    Stack,
    List,
    ListItem,
    ListItemButton,
    ListItemText,
    ListSubheader,
    Divider,
} from "@mui/material";
import { fetchServices } from "../../modules/api/toy-api";
import { CircularProgress } from "../../components/progress/CircularProgress";
import { SearchTextBox } from "../../components/SearchTextBox";
import { Resource, Result } from "../../modules/common";
import {
    ErrorMessage,
    ServiceSpec,
    ServiceSpecList,
    PortType,
} from "../../modules/api";

interface ServiceGridProps {
    resource: Resource<Result<ServiceSpecList, ErrorMessage>>;
}

function portTypeString(v: PortType): string {
    if (v.Source) {
        return "Source";
    } else if (v.Flow) {
        return "Flow";
    } else {
        return "Sink";
    }
}

const ServiceGrid = (props: ServiceGridProps) => {
    const r = props.resource.read();
    let namespaces: { [key: string]: ServiceSpec[] } = {};

    if (r.isSuccess()) {
        namespaces = r.value.items.reduce((acc, v) => {
            const ns = v.name_space;
            if (acc[ns]) {
                acc[ns].push(v);
            } else {
                acc[ns] = [v];
            }
            return acc;
        }, {});
    }

    return (
        <Box>
            {Object.entries(namespaces).map(([namespace, entry]) => {
                let displayNameSpace = namespace.replace("plugin.", "");
                return (
                    <Box sx={{ paddingTop: 1 }}>
                        <Typography key={displayNameSpace} variant="h6">
                            {displayNameSpace}
                        </Typography>
                        <Divider />
                        <Grid key={namespace} container spacing={2} m={1}>
                            {entry.map((item) => {
                                return (
                                    <Grid key={item.service_type} item xs={4}>
                                        <Card sx={{ maxWidth: 250 }}>
                                            <CardContent>
                                                <Typography variant="h6">
                                                    {item.service_name}
                                                </Typography>
                                                <Typography
                                                    sx={{ mb: 1.5 }}
                                                    color="text.secondary"
                                                >
                                                    {portTypeString(
                                                        item.port_type
                                                    )}
                                                </Typography>
                                                <Typography
                                                    variant="body2"
                                                    color="text.secondary"
                                                >
                                                    This impressive paella is a
                                                    perfect party
                                                </Typography>
                                            </CardContent>
                                        </Card>
                                    </Grid>
                                );
                            })}
                        </Grid>
                    </Box>
                );
            })}
        </Box>
    );
};

export interface ServiceListProps {
    open: boolean;
    onClose: () => void;
}

const style = {
    position: "relative" as "relative",
    top: "50%",
    left: "50%",
    transform: "translate(-50%, -50%)",
    width: "100%",
    maxWidth: "1100px",
    bgcolor: "background.paper",
    border: "2px solid #000",
    boxShadow: 24,
    p: 4,
};

export const ServiceSelector = (props: ServiceListProps) => {
    const [serviceResource, setServiceResource] = React.useState(() =>
        fetchServices()
    );

    const [searchText, setSearchText] = React.useState(() => "");
    const onSearchTextChange = React.useCallback((value: string) => {
        setSearchText(value);
    }, []);

    return (
        <Modal
            open={props.open}
            onClose={props.onClose}
            aria-labelledby="modal-modal-title"
            aria-describedby="modal-modal-description"
            sx={{ zIndex: 30000 }}
        >
            <Box sx={style} m={1}>
                <Stack direction="row" spacing={2}>
                    <Box sx={{ width: 350, maxWidth: 350 }}>
                        <List
                            component="nav"
                            subheader={
                                <ListSubheader
                                    component="div"
                                    id="services-list"
                                >
                                    Services
                                </ListSubheader>
                            }
                        >
                            <ListItem>
                                <SearchTextBox
                                    value={searchText}
                                    onChange={onSearchTextChange}
                                />
                            </ListItem>
                            <Divider />
                            <ListItem>
                                <ListItemButton>
                                    <ListItemText primary="source" />
                                </ListItemButton>
                            </ListItem>
                            <ListItem>
                                <ListItemButton>
                                    <ListItemText primary="flow" />
                                </ListItemButton>
                            </ListItem>
                            <ListItem>
                                <ListItemButton>
                                    <ListItemText primary="sink" />
                                </ListItemButton>
                            </ListItem>
                            <ListItem>
                                <ListItemButton>
                                    <ListItemText primary="all" />
                                </ListItemButton>
                            </ListItem>
                        </List>
                    </Box>
                    <Box
                        sx={{
                            overflow: "auto",
                            maxHeight: "80vh",
                            width: "100%",
                            paddingTop: 1,
                        }}
                    >
                        <React.Suspense fallback={<CircularProgress />}>
                            <ServiceGrid resource={serviceResource} />
                        </React.Suspense>
                    </Box>
                </Stack>
            </Box>
        </Modal>
    );
};

export default ServiceSelector;
