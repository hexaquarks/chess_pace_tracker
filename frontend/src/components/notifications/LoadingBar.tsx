import React from 'react';
import ProgressBar from '@ramonak/react-progress-bar';

interface LoadingBarProps {
    progress: string;
}

const LoadingBar: React.FC<LoadingBarProps> = ({ progress }) => {
    const [currentGameStr, totalGamesStr] = progress.replace('Game ', '').split('/');
    const currentGame = parseInt(currentGameStr) || 0;
    const totalGames = parseInt(totalGamesStr) || 1; // Avoid division by zero
    const progressValue = (currentGame / totalGames) * 100;

    return (
        <div className="flex flex-col bg-gray-800 p-6 rounded-lg shadow-lg w-2/3 max-w-2xl text-white">
            <div className="mb-2">
                <h3 className="text-lg font-bold">Loading games...</h3>
                <p className="text-sm">Game {currentGame} out of {totalGames} have been loaded</p>
            </div>
            <ProgressBar
                completed={progressValue}
                bgColor="#1A56DB"
                baseBgColor="#2a2a2a"
                height="20px"
                width="100%"
                borderRadius="8px"
                isLabelVisible={false}
                transitionDuration="200ms"
            />
        </div>
    );
};

export default LoadingBar;
