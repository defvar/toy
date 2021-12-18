import * as React from "react";
import MuiCircularProgress from "@mui/material/CircularProgress";
import Box from "@mui/material/Box";

export const CircularProgress = () => {
    return (
        <Box
            sx={{
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
            }}
        >
            <MuiCircularProgress />
        </Box>
    );
};

export default CircularProgress;
