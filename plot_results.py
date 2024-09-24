import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

df = pd.read_csv('resources/results.csv')

# Convert time columns from string to float (removing 's' suffix)
df['average_proof_time'] = df['average_proof_time'].str.replace('s', '').astype(float)
df['average_verification_time'] = df['average_verification_time'].str.replace('s', '').astype(float)

# Set the x-axis as the logarithm of total keys
x_values = np.log2(df['total_keys'])

# Create the plot
fig, ax1 = plt.subplots(figsize=(10, 6))

# Plotting the proving and verification times
ax1.plot(x_values, df['average_proof_time'], label='Average Proof Time', color='blue', marker='o')
ax1.plot(x_values, df['average_verification_time'], label='Average Verification Time', color='red', marker='o')
ax1.set_xlabel('Total Keys', fontsize=14)
ax1.set_ylabel('Time (s)', fontsize=14)
ax1.tick_params(axis='y', labelcolor='black')

# Adding grid lines for better readability
ax1.grid()

# Create a second y-axis for the proof size
ax2 = ax1.twinx()
ax2.plot(x_values, df['average_size'], label='Average Proof Size', color='green', marker='o')
ax2.set_ylabel('Proof Size (Bytes)', fontsize=14)
ax2.tick_params(axis='y', labelcolor='black')

# Annotate the x-axis with appropriate powers of 2
ax1.set_xticks(x_values)
ax1.set_xticklabels([f'$2^{{{int(np.log2(key))}}}$' for key in df['total_keys']])

# Set x-axis limits from 0 to 15
ax1.set_ylim(0, 15)

# Title and legend
plt.title(f'{str(df["name"][0]).capitalize()} Tree Benchmarking (Number of Runs: {df["no_runs"][0]})', fontsize=16)
ax1.legend(loc='upper left')
ax2.legend(loc='upper right')

# Show the plot
plt.tight_layout()
plt.show()