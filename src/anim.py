import numpy as np
import matplotlib.pyplot as plt
import matplotlib.animation as animation

import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

match_data = pd.read_json('C:/Users/gsala/Documents/Tekken7Replays/9c735ae0-eb36-4b82-b30b-442535893892.json')

TWOPI = 2*np.pi

fig, ax = plt.subplots()
plot, = plt.plot([match_data.iloc[0].to_frame().T['p1_x']], [match_data.iloc[0].to_frame().T['p1_z']], 'ro')

def animate(i):
    i = int(i)
    plot.set_data(match_data.iloc[i].to_frame().T['p1_x'], match_data.iloc[i].to_frame().T['p1_z'])
    print('update')
    return plot,

# create animation using the animate() function
myAnimation = animation.FuncAnimation(fig, animate, frames=np.arange(0, 3000, 1000), \
                                      interval=60, blit=True, repeat=True)

plt.show()