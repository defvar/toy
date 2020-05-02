import * as React from 'react';
import { makeStyles } from '@material-ui/core/styles';
import Paper from '@material-ui/core/Paper';
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableContainer from '@material-ui/core/TableContainer';
import TableHead from '@material-ui/core/TableHead';
import TablePagination from '@material-ui/core/TablePagination';
import TableRow from '@material-ui/core/TableRow';
import Switch from '@material-ui/core/Switch';
import GraphListMenu from './Menu';
import Labels from './Labels';

export interface GraphListItem {
    name: string,
    labels: string[],
    isActive: boolean,
}

export interface GraphListProps {
    items: { [key: string]: GraphListItem },
    onChangeActive: (name: string, active: boolean) => void
}

const useStyles = makeStyles({
    root: {
        width: '100%',
    },
    container: {
        maxHeight: 440,
    },
});

export const GraphList = (props: GraphListProps) => {
    const classes = useStyles();

    const [page, setPage] = React.useState(0);
    const [rowsPerPage, setRowsPerPage] = React.useState(10);

    const handleChangePage = (event: unknown, newPage: number) => {
        setPage(newPage);
    };

    const handleChangeRowsPerPage = (event: React.ChangeEvent<HTMLInputElement>) => {
        setRowsPerPage(+event.target.value);
        setPage(0);
    };

    const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        props.onChangeActive(event.target.name, event.target.checked);
    };

    return (
        <Paper className={classes.root}>
            <TableContainer className={classes.container}>
                <Table stickyHeader aria-label="sticky table">
                    <TableHead>
                        <TableRow>
                            <TableCell key="name" align="left" style={{ minWidth: 100 }} >
                                name
                            </TableCell>
                            <TableCell key="labels" align="left" style={{ minWidth: 100 }} >
                                labels
                            </TableCell>
                            <TableCell key="isActive" align="center" style={{ minWidth: 100 }} >
                                active
                            </TableCell>
                            <TableCell key="menu" align="center" style={{ minWidth: 100 }} >
                                menu
                            </TableCell>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        {Object.keys(props.items).slice(page * rowsPerPage, page * rowsPerPage + rowsPerPage).map((key) => {
                            const item = props.items[key];
                            return (
                                <TableRow hover role="checkbox" tabIndex={-1} key={item.name}>
                                    <TableCell key="name" align="left">
                                        {item.name}
                                    </TableCell>
                                    <TableCell key="labels" align="left">
                                        <Labels labels={item.labels.map(x => { return { label: x }; })} />
                                    </TableCell>
                                    <TableCell key="isActive" align="center">
                                        <Switch
                                            checked={item.isActive}
                                            name={key}
                                            onChange={handleChange}
                                            inputProps={{ 'aria-label': 'secondary checkbox' }}
                                        />
                                    </TableCell>
                                    <TableCell key="menu" align="center">
                                        <GraphListMenu name={key} />
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
    )
};
