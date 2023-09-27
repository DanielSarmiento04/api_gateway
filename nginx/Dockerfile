# Use the official Nginx image
FROM nginx:latest

# # Remove the default Nginx configuration
RUN rm /etc/nginx/conf.d/default.conf

# # Copy your custom Nginx configuration
COPY nginx.conf /etc/nginx/conf.d/

# Copy the SSL certificates
COPY ser.key /etc/nginx/
COPY ser.pem /etc/nginx/

# Expose ports
EXPOSE 80 443

# Set the default command to execute
CMD ["nginx", "-g", "daemon off;"]

