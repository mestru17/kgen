# Stage 1: Build Application
from maven:3.8-amazoncorretto-11 AS build

ARG MAVEN_REPO_URL
ARG MAVEN_REPO_USERNAME
ARG MAVEN_REPO_PASSWORD

COPY settings.xml /usr/share/maven/ref/settings.xml

ENV MAVEN_REPO_URL=${MAVEN_REPO_URL} \
    MAVEN_REPO_USERNAME=${MAVEN_REPO_USERNAME} \
    MAVEN_REPO_PASSWORD=${MAVEN_REPO_PASSWORD}

WORKDIR /build
COPY . .
RUN --mount=type=cache,target=/root/.m2 \
    mvn \
    --batch-mode \
    --settings /usr/share/maven/ref/settings.xml \
    clean package

# Stage 2: Create the Runtime Image
FROM amazoncorretto:11 AS runtime
WORKDIR /app
COPY --from=build /build/target/*-with-dependencies.jar app.jar
COPY application.yml /app/application.yml
ENTRYPOINT ["java", "-jar", "app.jar"]
