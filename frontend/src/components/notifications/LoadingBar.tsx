import React from 'react';
import useWebSocketHook from '../../hooks/useWebSocketHook';

const LoadingBar = () => {
    const { progress } = useWebSocketHook();

    return (
        <div>
            <h3>Loading Games</h3>
            <div>{progress}</div>
        </div>
    );
};

export default LoadingBar;