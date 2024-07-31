import React from 'react';
import ProgressBar from '@ramonak/react-progress-bar';

interface LoadingBarProps {
    progress: string;
}

const LoadingBar: React.FC<LoadingBarProps> = ({ progress }) => {
    const [currentGameStr, totalGamesStr] = progress.replace('Game ', '').split('/');
    const currentGame = parseInt(currentGameStr) || 0; // TODO: At first iteration its NaN, investigate
    const totalGames = parseInt(totalGamesStr) || 0; // TODO: At first iteration its NaN, investigate
    const progressValue = totalGames > 0 ? (currentGame / totalGames) * 100 : 0;

    return (
        <div className="flex flex-col bg-gray-800 p-6 rounded-lg shadow-lg w-2/3 max-w-2xl text-white">
            <div className="mb-2">
                <h3 className="text-lg font-bold">Loading games...</h3>
                <p className="text-sm">Game {currentGame} out of {totalGames} has been loaded</p>
            </div>
            <ProgressBar
                completed={progressValue}
                bgColor="#00FFFF"
                baseBgColor="#2a2a2a"
                height="10px"
                width="100%"
                borderRadius="8px"
            />
        </div>
    );
};

export default LoadingBar;