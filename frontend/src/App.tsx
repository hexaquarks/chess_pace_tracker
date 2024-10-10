import { useState, useEffect } from 'react';

import ResponsePanel from './components/panels/ResponsePanel';
import ErrorsPanel from './components/panels/ErrorsPanel';
import WinRateDonutChart from './components/charts/WinRateDonutChart';
import ToastAlertContainer from './components/notifications/ToastAlertContainer';
import ClipLoader from "react-spinners/ClipLoader";
import LoadingBar from './components/notifications/LoadingBar'

import { InputsPanel } from "./components/panels/InputsPanel";
import { ChessDataProvider, useChessData } from './context/ChessDataContext';
import { FlagPanel, extractFlagginInfoFromResponse } from './components/panels/FlagPanel';
import { DataSeriesChart, convertTrendChartData } from './components/charts/DataSeriesChart';
import { ToastAlertProps } from './components/notifications/ToastAlert';

const AppContent: React.FC = () => {
    const { response, isLoading, error, fetchData, usernameNotFound, fetchProgress } = useChessData();
    const [toasts, setToasts] = useState<ToastAlertProps[]>([]);
    const [showLoading, setShowLoading] = useState(false);

    const removeToast = (id: number) => {
        setToasts((prevToasts: ToastAlertProps[]) => prevToasts.filter((toast) => toast.id !== id));
    };

    const addToast = (message: string) => {
        const newToast: ToastAlertProps = { id: Date.now(), message, removeToast };
        setToasts((prevToasts: ToastAlertProps[]) => [...prevToasts, newToast]);
    };

    useEffect(() => {
        if (error) {
            addToast(error);
        }
    }, [error]);

    // Implementation to ensure that the loading bar is shown for at least 1 second after response 
    useEffect(() => {
        if (isLoading) {
            setShowLoading(true);
        } else if (!isLoading && response) {
            setTimeout(() => setShowLoading(false), 750);
        }
    }, [isLoading, response]);

    return (
        <div>
            {showLoading && (
                <div className="fixed inset-0 bg-black bg-opacity-30 flex justify-center items-center z-50">
                    {/* <ClipLoader color="#FFFFFF" loading={isLoading} size={75} /> */}
                    <LoadingBar progress={fetchProgress} />
                </div>
            )}
            <ToastAlertContainer
                toasts={toasts}
                removeToast={removeToast}
            />

            {/* Main page */}
            <div className="flex flex-col items-center bg-zinc-900 pt-20 pb-8 min-h-screen sm:px-20 px-3">
                {/* Components container */}
                <div className="flex flex-col lg:flex-row justify-center w-full max-w-7xl">
                    {/* Left column with components */}
                    <div className="w-full lg:w-3/4 lg:max-w-2xl px-4">
                        <InputsPanel
                            handleSendData={fetchData}
                            usernameNotFound={usernameNotFound}
                        />
                        {response && (
                            <div>
                                <ResponsePanel
                                    time={response.time}
                                    explanationMessage={response.explanation_message}
                                />
                                <DataSeriesChart
                                    key={response.unique_key}
                                    {...convertTrendChartData(response.trend_chart_data)}
                                />
                            </div>
                        )}
                    </div>

                    {/* Right column with components */}
                    {response && (
                        <div className="w-full lg:w-1/4 px-4 lg:px-0">
                            <ErrorsPanel
                                gamesWithError={response.games_with_errors}
                                totalNumberOfGames={response.games_with_errors.length} // TODO: Inceonsistency here 
                            />
                            <div className="w-full max-w-sm mx-auto lg:py-0 pt-10">
                                <WinRateDonutChart
                                    winRate={response.player_win_rate_in_fetched_games}
                                />
                            </div>
                            <FlagPanel
                                key={response.unique_key}
                                {...extractFlagginInfoFromResponse(response)}
                            />
                        </div>
                    )}
                </div>
            </div>

        </div>
    );
}

const App = () => {
    return (
        <ChessDataProvider>
            <AppContent />
        </ChessDataProvider>
    );
}

export default App;
