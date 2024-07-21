import { STATUS_CODES } from "http";

interface RequestInformation {
	username: string;
	games_count: number;
	game_mode: string;
	user_color: string;
}

export enum MessageInformationAssessment {
	Positive,
	Neutral,
	Negative
}

export interface TrendChartDatum {
	time_differential: number,
	win_status: string,
	game_number: number
}

interface ResponseInformationInternal {
	time: number;
	explanation_message: [string, MessageInformationAssessment]
	games_with_errors: Array<[number, string]>
	trend_chart_data: [TrendChartDatum]
	player_win_rate_in_fetched_games: number
	players_flag_counts: [number, number]
}

interface UniqueIdentifier {
	unique_key: number
}

export type ResponseInformation = ResponseInformationInternal & UniqueIdentifier;

export const sendDataToBackend = async (
	username: string,
	gamesCount: number,
	gameMode: string,
	userColor: string,
	setUsernameNotFound: (e: boolean) => void
)
: Promise<Readonly<ResponseInformation>> => {
	try {
		const payload: RequestInformation = {
			username,
			games_count: gamesCount,
			game_mode: gameMode,
			user_color: userColor
		};
		console.log('Sending:', payload);

		const response = await fetch('http://localhost:8000/fetch-chess-data', {
			method: 'POST',
			body: JSON.stringify(payload),
			headers: {
				'Content-Type': 'application/json',
				"x-requested-by": "frontend"
			},
		});

		if (!response.ok) {
			const errorStatus = await response.status;
			handleErrorResponse(errorStatus, setUsernameNotFound);
		} else {
			setUsernameNotFound(false);
		}

		const data: ResponseInformationInternal = await response.json();
		console.log(data);

		return { ...data, unique_key: Date.now() };

	} catch (error) {
		console.error('Error sending data to backend', error);
		throw error;
	}
}

const handleErrorResponse = (
	status: number, 
	setUsernameNotFound: (e: boolean) => void
) => {
    switch (status) {
        case 400:
			throw new Error(`The request was invalid with status ${status}.`);
        case 401:
			throw new Error('Unauthorized');
        case 403:
			throw new Error('You do not have permission to access this resource.');
        case 404:
			setUsernameNotFound(true);
			throw new Error(`The username was not found. Make sure the username is correct and try again.`);
        case 500:
			throw new Error('An internal server error occured. Please try again later.');
        case 503:
			throw new Error('The service is unavailable right now. Please try again later.');
        default:
			throw new Error(`An unexpected error occured with status: ${status}.`);
    }
}