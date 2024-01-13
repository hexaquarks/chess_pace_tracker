interface RequestInformation {
    username: string;
    games_count: number;
  }
  
  export const sendDataToBackend = async (
    username: string, 
    gamesCount: number,
    gameMode: string,
    userColor: string) => {
    try {
      const payload: RequestInformation = {
        username,
        games_count: gamesCount,
      };
      console.log('Sending:', payload);
  
      const response = await fetch('http://localhost:8000/fetch-chess-data', {
        method: 'POST',
        body: JSON.stringify(payload),
        headers: {
          'Content-Type': 'application/json',
        },
      });
  
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
  
      const data = await response.json();
      console.log(data);
    } catch (error) {
      console.error('Error sending data to backend', error);
    }
  };
  