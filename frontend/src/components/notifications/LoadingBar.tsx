import React from 'react';

interface LoadingBarProps {
    progress: string;
}

const LoadingBar: React.FC<LoadingBarProps> = ({ progress }) => {
    return (
        <div>
            <h3 color="#FFFFFF">Loading Games</h3>
            <div color="#FFFFFF">{progress}</div>
        </div>
    );
};

export default LoadingBar;
