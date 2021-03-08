import { Injectable, OnModuleInit } from "@nestjs/common";
import { rabbitmqKeys } from "../common/Configuration";
import { AnswerQuestionDto } from "./dto/Answer.dto";
import { FinishExamDto } from "./dto/FinishExam.dto";
import { StartExamDto } from "./dto/StartExam.dto";
import * as amqplib from "amqplib";

const QUEUE_PATTERN_EXAM = "q_patterns";
const PATTERN_START_EXAM = "exam_started";
const PATTERN_ANSWER_QUESTION = "question_answered";
const PATTERN_FINISH_EXAM = "exam_finished";
const PATTERNS = [PATTERN_START_EXAM, PATTERN_ANSWER_QUESTION, PATTERN_FINISH_EXAM] as const;

type Patterns = typeof PATTERNS[number];

const { host, password, port, username, vhost } = rabbitmqKeys;

const generateUuid = () => Math.random().toString() + Math.random().toString() + Math.random().toString();

@Injectable()
export class ExamQueueService implements OnModuleInit {
    private connection: amqplib.Connection;

    public constructor() { }

    public async onModuleInit() {
        this.connection = await amqplib.connect(`amqp://${username}:${password}@${host}:${port}/${vhost}`);
    }

    public async send(pattern: Patterns, data: StartExamDto | AnswerQuestionDto | FinishExamDto): Promise<any> {
        const channel = await this.connection.createChannel();
        const q = await channel.assertQueue("", {
            durable: false,
            exclusive: false,
            autoDelete: true,
        });
        const correlationId = generateUuid();

        const payload = {
            pattern,
            data
        };
        console.info("Client send: ", payload);

        channel.sendToQueue(
            QUEUE_PATTERN_EXAM,
            Buffer.from(JSON.stringify(payload)),
            {
                correlationId: correlationId,
                replyTo: q.queue
            }
        );

        return new Promise((resolve) => {
            channel.consume(q.queue, (msg) => {
                if (msg.properties.correlationId == correlationId) {
                    resolve(msg.content.toString());
                }
            }, {
                noAck: true,
            })
        });
    }

    public async sendStartExam(startExamDto: StartExamDto): Promise<any> {
        return this.send(PATTERN_START_EXAM, startExamDto);
    }

    public sendQuestionAnswer(answerQuestionDto: AnswerQuestionDto): Promise<any> {
        return this.send(PATTERN_ANSWER_QUESTION, answerQuestionDto);
    }

    public sendfinishExam(finishExamDto: FinishExamDto): Promise<string> {
        return this.send(PATTERN_FINISH_EXAM, finishExamDto);
    }
}