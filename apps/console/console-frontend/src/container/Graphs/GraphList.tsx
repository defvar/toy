import * as React from "react";
import { makeStyles, createStyles, Theme } from "@material-ui/core/styles";
import Paper from "@material-ui/core/Paper";
import Table from "@material-ui/core/Table";
import TableBody from "@material-ui/core/TableBody";
import TableCell from "@material-ui/core/TableCell";
import TableContainer from "@material-ui/core/TableContainer";
import TableHead from "@material-ui/core/TableHead";
import TablePagination from "@material-ui/core/TablePagination";
import TableRow from "@material-ui/core/TableRow";
import Switch from "@material-ui/core/Switch";
import LinearProgress from "@material-ui/core/LinearProgress";
import { SimpleMenu, SimpleMenuProps } from "../../components/SimpleMenu";
import { LabelChips } from "../../components/LabelChips";
import { GraphListItemState, Actions } from "../../modules/graphs";
import { useHistory } from "react-router-dom";

export interface GraphListProps {
    items: { [key: string]: GraphListItemState };
    dispatch: React.Dispatch<Actions>;
}

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            width: "100%",
        },
        container: {
            maxHeight: 440,
        },
        tableHeader: {},
        rowProgress: {},
    })
);

const menuOptions = (history, name): SimpleMenuProps => {
    return {
        options: [
            {
                display: "Edit",
                onClick: () => {
                    history.push(`/graphs/${name}/edit`);
                },
            },
            {
                display: "Log",
                onClick: () => {
                    console.log("log");
                },
            },
        ],
    };
};

export const GraphList = (props: GraphListProps) => {
    const classes = useStyles();
    const history = useHistory();

    const [page, setPage] = React.useState(0);
    const [rowsPerPage, setRowsPerPage] = React.useState(10);
    const [loadingItems, setLoadingItems] = React.useState({});

    const handleChangePage = (event: unknown, newPage: number) => {
        setPage(newPage);
    };

    const handleChangeRowsPerPage = (
        event: React.ChangeEvent<HTMLInputElement>
    ) => {
        setRowsPerPage(+event.target.value);
        setPage(0);
    };

    const ref = React.useRef(null);
    ref.current = "mount";
    React.useEffect(() => {
        return () => (ref.current = null);
    }, [ref]);
    const toggleActive = React.useCallback(
        async (event: React.ChangeEvent<HTMLInputElement>) => {
            const { name, checked: isActive } = event.target;
            setLoadingItems((prev) => ({ ...prev, [name]: true }));
            const promise = new Promise((resolve) => {
                setTimeout(() => {
                    resolve();
                }, 2000);
            });
            await promise;
            if (ref.current) {
                props.dispatch({
                    type: "ToggleActive",
                    payload: { name, isActive },
                });
            }
            setLoadingItems((prev) => {
                const r = { ...prev };
                delete r[name];
                return r;
            });
        },
        [ref]
    );

    return (
        <Paper className={classes.root}>
            <TableContainer className={classes.container}>
                <Table stickyHeader aria-label="sticky table">
                    <TableHead className={classes.tableHeader}>
                        <TableRow>
                            <TableCell
                                key="name"
                                align="left"
                                style={{ minWidth: 100 }}
                            >
                                name
                            </TableCell>
                            <TableCell
                                key="labels"
                                align="left"
                                style={{ minWidth: 100 }}
                            >
                                labels
                            </TableCell>
                            <TableCell
                                key="isActive"
                                align="center"
                                style={{ minWidth: 100 }}
                            >
                                active
                            </TableCell>
                            <TableCell
                                key="menu"
                                align="center"
                                style={{ minWidth: 100 }}
                            >
                                menu
                            </TableCell>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        {Object.keys(props.items)
                            .slice(
                                page * rowsPerPage,
                                page * rowsPerPage + rowsPerPage
                            )
                            .map((key) => {
                                const item = props.items[key];
                                const isProgress = key in loadingItems;
                                return (
                                    <TableRow
                                        hover
                                        role="checkbox"
                                        tabIndex={-1}
                                        key={item.name}
                                    >
                                        <TableCell key="name" align="left">
                                            {item.name}
                                            {isProgress && (
                                                <LinearProgress
                                                    key={`${item.name}-progress`}
                                                    className={
                                                        classes.rowProgress
                                                    }
                                                />
                                            )}
                                        </TableCell>
                                        <TableCell key="labels" align="left">
                                            <LabelChips
                                                labels={item.labels.map(
                                                    (x) => ({
                                                        key: x,
                                                        display: x,
                                                    })
                                                )}
                                            />
                                        </TableCell>
                                        <TableCell
                                            key="isActive"
                                            align="center"
                                        >
                                            <Switch
                                                checked={item.isActive}
                                                disabled={isProgress}
                                                name={key}
                                                onChange={toggleActive}
                                                inputProps={{
                                                    "aria-label":
                                                        "secondary checkbox",
                                                }}
                                            />
                                        </TableCell>
                                        <TableCell key="menu" align="center">
                                            <SimpleMenu
                                                {...menuOptions(
                                                    history,
                                                    item.name
                                                )}
                                            />
                                        </TableCell>
                                    </TableRow>
                                );
                            })}
                    </TableBody>
                </Table>
            </TableContainer>
            <TablePagination
                rowsPerPageOptions={[10, 25, 100]}
                component="div"
                count={Object.keys(props.items).length}
                rowsPerPage={rowsPerPage}
                page={page}
                onChangePage={handleChangePage}
                onChangeRowsPerPage={handleChangeRowsPerPage}
            />
        </Paper>
    );
};
