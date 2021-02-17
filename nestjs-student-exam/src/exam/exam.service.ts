import { Injectable } from '@nestjs/common';
import { ExamDto } from './dto/Exam.dto';
import { ExamQueueService } from './examQueue.service';

@Injectable()
export class ExamService {
  public constructor(private readonly examQueueService: ExamQueueService) { }

  public getHello(): string {
    return 'Hello World!';
  }

  public async emitStartExam(examData: ExamDto): Promise<any> {
    return this.examQueueService.sendStartQueue(examData).toPromise();
  }
}
