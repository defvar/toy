import * as React from "react";
import * as MatAccountCircle from "@mui/icons-material/AccountCircle";
import IconButton from "@mui/material/IconButton";
import MenuItem from "@mui/material/MenuItem";
import Menu from "@mui/material/Menu";
import ListItemIcon from "@mui/material/ListItemIcon";
import ListItemText from "@mui/material/ListItemText";

export interface AccountCircleProps {
    options: {
        key: string;
        display: string;
        icon?: React.ReactNode;
    }[];
    onMenuItemClick: (key: string) => void;
}

export const AccountCircle = ({
    options,
    onMenuItemClick,
}: AccountCircleProps) => {
    const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
    const circleOpen = Boolean(anchorEl);

    const handleCircle = (event: React.MouseEvent<HTMLElement>) => {
        setAnchorEl(event.currentTarget);
    };

    const handleCircleClose = () => {
        setAnchorEl(null);
    };

    const handleMenuItemClick = (key: string) => {
        setAnchorEl(null);
        onMenuItemClick(key);
    };

    return (
        <div>
            <IconButton
                aria-label="account of current user"
                aria-controls="menu-appbar"
                aria-haspopup="true"
                onClick={handleCircle}
                color="inherit"
                size="large">
                <MatAccountCircle.default />
            </IconButton>
            <Menu
                id="menu-appbar"
                anchorEl={anchorEl}
                anchorOrigin={{
                    vertical: "bottom",
                    horizontal: "left",
                }}
                keepMounted
                transformOrigin={{
                    vertical: "top",
                    horizontal: "left",
                }}
                open={circleOpen}
                onClose={handleCircleClose}
            >
                {options.map((option) => {
                    return (
                        <MenuItem
                            key={option.key}
                            onClick={() => handleMenuItemClick(option.key)}
                        >
                            <ListItemIcon>{option.icon}</ListItemIcon>
                            <ListItemText primary={option.display} />
                        </MenuItem>
                    );
                })}
            </Menu>
        </div>
    );
};

export default AccountCircle;
