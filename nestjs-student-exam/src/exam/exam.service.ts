import { Injectable } from '@nestjs/common';
import { AnswerQuestionDto } from './dto/Answer.dto';
import { FinishExamDto } from './dto/FinishExam.dto';
import { StartExamDto } from './dto/StartExam.dto';
import { ExamQueueService } from './examQueue.service';

@Injectable()
export class ExamService {
  public constructor(private readonly examQueueService: ExamQueueService) { }

  public async emitStartExam(exam: StartExamDto): Promise<any> {
    return this.examQueueService.sendStartExam(exam);
  }

  public async sendQuestionAnswer(questionAnswer: AnswerQuestionDto): Promise<any> {
    return this.examQueueService.sendQuestionAnswer(questionAnswer);
  }

  public async finishExam(finishExamDto: FinishExamDto): Promise<any> {
    return this.examQueueService.sendfinishExam(finishExamDto);
  }
}
