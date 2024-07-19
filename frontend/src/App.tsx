import { useState } from 'react';

import ResponsePanel from './components/panels/ResponsePanel';
import ErrorsPanel from './components/panels/ErrorsPanel';
import WinRateDonutChart from './components/charts/WinRateDonutChart';
import ToastAlertContainer from './components/notifications/ToastAlertContainer';
import ClipLoader from "react-spinners/ClipLoader";
import { InputsPanel, InputProps } from "./components/panels/InputsPanel";

import { sendDataToBackend, ResponseInformation } from './services/apiService';
import { FlagPanel, extractFlagginInfoFromResponse } from './components/panels/FlagPanel';
import { DataSeriesChart, convertTrendChartData } from './components/charts/DataSeriesChart';
import { ToastAlertProps } from './components/notifications/ToastAlert';

const App = () => {
	const [response, setResponse] = useState<ResponseInformation | null>(null);
	const [toasts, setToasts] = useState<ToastAlertProps[]>([]);
	const [isLoading, setIsLoading] = useState(false);

	const handleSendData = async (props: InputProps) => {
		console.log("in here ")
		try {
			setIsLoading(true);
			const responseData: ResponseInformation = await sendDataToBackend(
				props.username,
				props.gamesCount,
				props.gameMode,
				props.userColor);
			setResponse(responseData);
		} catch (error) {
			addToast(error instanceof Error ? error.message : 'An unexpected error occurred');
		} finally {
			setIsLoading(false);
		}
	};

	const removeToast = (id: number) => {
		setToasts((prevToasts: ToastAlertProps[]) => prevToasts.filter((toast) => toast.id !== id));
	};

	const addToast = (message: string) => {
		const newToast: ToastAlertProps = { id: Date.now(), message, removeToast };
		setToasts((prevToasts: ToastAlertProps[]) => [...prevToasts, newToast]);
	};

	return (
		<div>
			{isLoading && (
				<div className="fixed inset-0 bg-black bg-opacity-30 flex justify-center items-center z-50">
					<ClipLoader color="#FFFFFF" loading={isLoading} size={75} />
				</div>
			)}
			<ToastAlertContainer toasts={toasts} removeToast={removeToast} />
			<div className="flex flex-col items-center justify-between w-screen bg-zinc-900 pt-20 pb-8 min-h-screen">
				<div className="flex flex-row justify-center w-full max-w-7xl">
					<div className="flex flex-col w-full max-w-2xl px-4">
						<InputsPanel handleSendData={handleSendData} />
						{
							response && (
								<div>
									<div className="bg-gray-800 p-6 rounded-lg shadow-md">
										<ResponsePanel
											time={response.time}
											explanationMessage={response.explanation_message}
										/>
									</div>
									<div>
										<DataSeriesChart key={response.unique_key} {...convertTrendChartData(response.trend_chart_data)} />
									</div>
								</div>
							)
						}

					</div>

					{response && (
						<div className="w-1/4 flex flex-col">
							<ErrorsPanel
								gamesWithError={response.games_with_errors}
								totalNumberOfGames={response.games_with_errors.length} // temporary
							/>
							<WinRateDonutChart winRate={response.player_win_rate_in_fetched_games} />
							<FlagPanel key={response.unique_key} {...extractFlagginInfoFromResponse(response)} />
						</div>
					)}
				</div>
			</div>
		</div>
	);
}

export default App;
